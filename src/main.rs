use std::path::{Path, PathBuf};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Initial directory
    #[arg(long)]
    old_dir: String,

    /// Directory to move to
    #[arg(long)]
    new_dir: String,
}


struct Output {
    commands: Vec<String>,
}


impl Output {
    fn new() -> Output {
        Output{ commands: Vec::new() }
    }

    /// send shell commands to execute to autocd
    fn send(self) {
        println!("{}", self.commands.join("; "));
    }

    fn push(&mut self, cmd: String) {
        self.commands.push(cmd);
    }
}


fn main() {
    let cli = Cli::parse();
    // let cwd = current_dir().unwrap();

    let source = PathBuf::from(cli.old_dir);
    let target = PathBuf::from(cli.new_dir);

    let mut output = Output::new();

    // only run .env.leave if getting out of directory, not deeper inside
    if !target.starts_with(&source) {
        // find all .env.leave files of source ancestor folders that are not ancestors of target
        for ancestor in source.ancestors() {
            if !target.starts_with(ancestor) {
                if let Some(f) = get_envleave_file(ancestor) {
                    output.push(shell_command::source(f.to_str().unwrap()));
                }
            }
        }
    }

    // run all parent folders env files
    for ancestor in target.ancestors() {
        if let Some(f) = get_env_file(ancestor) {
            output.push(shell_command::cd(ancestor.to_str().unwrap()));
            output.push(shell_command::source(f.to_str().unwrap()));
        };
    }

    output.push(shell_command::cd(target.to_str().unwrap()));

    output.send()
}


fn get_envleave_file(dir: &Path) -> Option<PathBuf> {
    let env_path = Path::new(dir).join(".env.leave");

    match env_path.is_file() {
        true => Some(env_path),
        false => None,
    }
}


fn get_env_file(dir: &Path) -> Option<PathBuf> {
    let env_path = Path::new(dir).join(".env");

    match env_path.is_file() {
        true => Some(env_path),
        false => None,
    }
}


mod shell_command {
    pub fn source(f: &str) -> String {
        format!("source {f}")
    }

    // TODO: parameterize cd?
    pub fn cd(dir: &str) -> String {
        format!("builtin cd {dir}")
    }
}
