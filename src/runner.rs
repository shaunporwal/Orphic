use crate::prompts::get_prompt;
use crate::utils::try_extract;
use crate::cli::CliOptions;
use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, Role}};
use execute::{Execute, shell};
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process::Stdio;

#[derive(Debug)]
struct UserAbort;

impl fmt::Display for UserAbort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User aborted command")
    }
}
impl Error for UserAbort {}

/// Verify/repair JSON via GPT when extraction failed.
async fn verify_json(client: &Client, input: &str) -> Result<Option<String>, Box<dyn Error>> {
    let history = vec![
        ChatCompletionRequestMessage {
            role: Role::System,
            content: String::from(get_prompt("json_verify_system")),
            name: None,
        },
        ChatCompletionRequestMessage {
            role: Role::User,
            content: format!("{}{}", get_prompt("json_verify_user"), input),
            name: None,
        },
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(crate::cli::GPT_35_TURBO)
        .messages(history)
        .build()?;

    let response = client.chat().create(request).await?;
    let body = response.choices[0].message.content.to_owned();

    Ok(match body.trim() {
        "" => None,
        _ => Some(body),
    })
}

async fn parse_command(client: &Client, body: &str) -> Result<Option<Value>, Box<dyn Error>> {
    match try_extract(body) {
        Some(commands) => Ok(Some(commands)),
        None => match verify_json(client, body).await? {
            Some(fixed) => Ok(try_extract(&fixed)),
            None => Ok(None),
        },
    }
}

async fn interpret(client: &Client, task: &str, output: &str, opts: CliOptions) -> Result<String, Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(opts.model)
        .messages(vec![
            ChatCompletionRequestMessage {
                role: Role::System,
                content: String::from(get_prompt("interpreter_system")),
                name: None,
            },
            ChatCompletionRequestMessage {
                role: Role::User,
                content: json!({"task": task, "output": output}).to_string()
                    + get_prompt("interpreter_user"),
                name: None,
            },
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    Ok(response.choices[0].message.content.to_owned())
}

async fn try_command(
    client: &Client,
    input: String,
    history: &mut Vec<ChatCompletionRequestMessage>,
    opts: CliOptions,
) -> Result<String, Box<dyn Error>> {
    history.push(ChatCompletionRequestMessage {
        role: Role::User,
        content: input + get_prompt("assistant_user"),
        name: None,
    });

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(opts.model)
        .messages(history.clone())
        .build()?;

    let response = client.chat().create(request).await?;
    let body = response.choices[0].message.content.to_owned();

    let mut final_output = match parse_command(client, &body).await? {
        Some(commands) => match commands["command"].as_str() {
            Some(command) => {
                if !opts.unsafe_mode {
                    let mut input = String::new();
                    println!("{}", command);
                    print!("Execute? [Y/n] ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut input)?;

                    match input.trim().to_lowercase().as_str() {
                        "" | "y" | "yes" => {}
                        _ => return Err(Box::new(UserAbort)),
                    }
                }

                let mut sh = shell(command);
                sh.stdout(if !opts.interpret {
                    Stdio::inherit()
                } else {
                    Stdio::piped()
                });
                String::from_utf8(sh.execute_output()?.stdout)? + "\n"
            }
            None => body.clone() + "\n",
        },
        None => body.clone() + "\n",
    };

    // Prepend raw GPT body when debug flag is set
    if opts.debug {
        final_output = format!("{}\n{}", body.trim_end(), final_output.trim_end());
        final_output.push('\n');
    }

    Ok(final_output)
}

async fn repl(client: &Client, opts: CliOptions) -> Result<(), Box<dyn Error>> {
    let mut history: Vec<ChatCompletionRequestMessage> = Vec::new();

    loop {
        let mut input = String::new();
        print!("orphic> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "quit" => break,
            task => {
                let res_maybe = try_command(client, task.to_string(), &mut history, opts).await;
                match res_maybe {
                    Ok(res) => {
                        history.push(ChatCompletionRequestMessage {
                            role: Role::Assistant,
                            content: res.clone(),
                            name: None,
                        });

                        if opts.interpret {
                            println!("{}", interpret(client, task, &res, opts).await?);
                        } else {
                            print!("{}", res.trim());
                        }
                    }
                    Err(e) => {
                        if e.is::<UserAbort>() {
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Entry point used by `main.rs`.
pub async fn execute(client: &Client, opts: CliOptions, task: String) -> Result<(), Box<dyn Error>> {
    println!("Using model: {}", opts.model);

    if opts.repl {
        repl(client, opts).await
    } else {
        let mut history: Vec<ChatCompletionRequestMessage> = Vec::new();
        let res = try_command(client, task.clone(), &mut history, opts).await?;

        if opts.interpret {
            println!("{}", interpret(client, &task, &res, opts).await?);
        } else {
            print!("{}", res.trim());
        }

        Ok(())
    }
} 