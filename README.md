# Orphic
*A natural language shell interface for \*nix systems.*

---
### Overview
Orphic is a CLI tool that uses OpenAI GPT models to translate natural-language tasks into shell commands executed on your system.  
As of the current version, **Orphic defaults to the `gpt-4-turbo` model**. You can explicitly choose a different model (e.g. `gpt-4` or `gpt-3.5-turbo`) with the `--model` flag detailed below.  
It excels at tasks that are relatively straightforward but require complex and obscure commands to accomplish.

```
$ orphic delete all installers or tars from ~/Downloads that are more than a week old
find ~/Downloads -name '*.tar.*' -mtime +7 -delete; find ~/Downloads -name '*.dmg' -mtime +7 -delete
Execute? [Y/n] Y
```

*Note: Orphic defaults to safe mode, and will not automatically execute commands without confirmation unless unsafe mode is specified.*

### Installation
* Make sure your system has rust and cargo.
* `cargo install orphic`
* Orphic requires the `OPENAI_API_KEY` environment variable to be set. You can generate one [here](https://openai.com/).

### Usage
Orphic is designed to be used like you would use any other CLI tool.

`$ orphic sort ~/Downloads into folders based on media type`

`$ orphic how strong is my network connection`

`$ orphic what version kernel am i running`

`$ orphic show me the name and size of all files larger than 8MB in ~/Downloads/` 

`$ orphic <do task that would otherwise require complex commands that you don't know off the top of your head>`

`-u` or `--unsafe` will execute commands without user verification.

`-m <MODEL>` or `--model <MODEL>` lets you pick which GPT model to use.  
Supported values: `gpt-4-turbo` (default), `gpt-4`, `gpt-3.5-turbo`.  
Example: ``orphic -m gpt-3.5-turbo "list all *.rs files"``

`-i` or `--interpret` will describe the output of the task in natural language (note that this is generally very slow).
```
$ orphic -u -i how much disk space is available
You have 16GB available out of a total of 113GB on your main hard 
drive, which is mounted on the root directory. 
Other partitions and file systems are also listed with their 
respective usage percentages and mount points.
```

`-d` or `--debug` prints the raw GPT response *in addition* to the normal result so you can see exactly what the model produced.

```
$ orphic -u -d count the lines of rust code in this directory excluding /target/.
{"command": "find . -name target -prune -o -name '*.rs' -type f -print0 | xargs -0 wc -l"}
61 ./src/prompts.rs
219 ./src/runner.rs
280 total
```

`-r` or `--repl` will start Orphic in a REPL environment.
```
$ orphic -u -r
orphic> when did i last login
wtmp begins Sat Mar 18 14:55
orphic> quit
$
```

### Optional Zsh command-not-found integration
Add the snippet below to the bottom of your `~/.zshrc` to let Orphic handle any *free-text* command you type.  When Zsh can't find a matching binary or alias, the handler checks whether you entered more than one word and, if so, forwards the whole line to Orphic.

```zsh
# Send unknown multi-word commands to Orphic
command_not_found_handler() {
  # If the user typed multiple words, treat the entire line as a natural-language task
  if [[ "$1" == *' '* || $# -gt 1 ]]; then
    orphic "$@"   # change to the exact binary name you installed
    return $?      # propagate Orphic's exit status
  fi

  # Fallback to the standard message for single-word typos
  print "zsh: command not found: $1" >&2
  return 127
}
```

**Benefits**
1. Keeps your prompt clean – just type "*list my large downloads from last week*" and hit Enter.
2. Falls back gracefully to Zsh's default behaviour for single-word typos.

After editing `~/.zshrc`, run `source ~/.zshrc` or start a new terminal session.

### Usage tips and observations 
Sometimes Orphic works. Sometimes it doesn't. GPT is inconsistent, and the prompts that I'm using leave a lot to be desired. Results seem to be better if you format your task as a command instead of a question ("list the currently open ports" instead of "what ports are currently open"). An error that often arises is that GPT will try to use commands or packages for a different OS/distribution, or will try to use tools that you don't currently have installed. A quick fix is to specify your OS if you think the task will require OS-specific tools, but I'm working on making Orphic more aware of which commands are at its disposal and which are not. 

### Contributing 
Pull requests welcome. If you use Orphic and get a good/interesting output, please send it to me. Likewise, if you get a really bad output, please also send it to me or open an issue. This system is very experimental and I'm still trying to figure out what works and what doesn't when it comes to prompts and configurations.

### License
[MIT](https://choosealicense.com/licenses/mit/)

Copyright (c) Will Savage, 2023

