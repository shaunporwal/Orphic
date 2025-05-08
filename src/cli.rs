use clap::{command, Arg, ArgAction};

/// Supported model constants (must have 'static lifetime for OpenAI client).
pub const GPT_35_TURBO: &str = "gpt-3.5-turbo";
pub const GPT_4: &str = "gpt-4";
pub const GPT_4_TURBO: &str = "gpt-4-turbo";

/// Command-line options supplied by the user.
#[derive(Clone, Copy, Debug)]
pub struct CliOptions {
    pub repl: bool,
    pub interpret: bool,
    pub unsafe_mode: bool,
    pub debug: bool,
    pub model: &'static str,
}

/// Parse command-line arguments and return `(CliOptions, task_string)`.
/// The task string is a single space-joined string of the positional `task`
/// arguments (the same behaviour as before).
pub fn parse() -> (CliOptions, String) {
    let matches = command!()
        .arg(Arg::new("task").action(ArgAction::Append))
        .arg(
            Arg::new("repl")
                .short('r')
                .long("repl")
                .action(ArgAction::SetTrue)
                .help("Start an interactive REPL"),
        )
        .arg(
            Arg::new("interpret")
                .short('i')
                .long("interpret")
                .action(ArgAction::SetTrue)
                .help("Interpret command output using GPT"),
        )
        .arg(
            Arg::new("unsafe")
                .short('u')
                .long("unsafe")
                .action(ArgAction::SetTrue)
                .help("Execute commands without confirmation prompt"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Display raw GPT output along with final result"),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_parser([GPT_35_TURBO, GPT_4, GPT_4_TURBO])
                .default_value(GPT_4_TURBO)
                .help("Specify the GPT model (default: gpt-4-turbo)"),
        )
        .get_matches();

    // Resolve model into a 'static str so we can keep it inside `CliOptions`.
    let selected_model = matches
        .get_one::<String>("model")
        .map(|s| s.as_str())
        .unwrap_or(GPT_4_TURBO);

    let model_static: &'static str = match selected_model {
        GPT_35_TURBO => GPT_35_TURBO,
        GPT_4 => GPT_4,
        _ => GPT_4_TURBO,
    };

    let opts = CliOptions {
        repl: matches.get_flag("repl"),
        interpret: matches.get_flag("interpret"),
        unsafe_mode: matches.get_flag("unsafe"),
        debug: matches.get_flag("debug"),
        model: model_static,
    };

    let task = matches
        .get_many::<String>("task")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    (opts, task)
} 