use std::path::{Path, PathBuf};
use clap::Parser;
mod shell;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Initial directory
    #[arg(long)]
    old_dir: String,

    /// Directory to move to
    #[arg(long)]
    new_dir: String,

    /// Verbose: print all sourced envs
    #[arg(short, long, default_value_t=false)]
    verbose: bool,
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
                    if cli.verbose {
                        output.push(shell::verbose(f.to_str().unwrap()));
                    }
                    output.push(shell::source(f.to_str().unwrap()));
                }
            }
        }
    }

    // push ancestors to vec because ancestors() does not implement rev()
    let mut ancestors = Vec::new();
    for ancestor in target.ancestors() {
        ancestors.push(ancestor);
    }

    // run all parent folders env files from ancestors to children
    for ancestor in ancestors.iter().rev() {
        if let Some(f) = get_env_file(ancestor) {
            if cli.verbose {
                output.push(shell::verbose(f.to_str().unwrap()));
            }
            output.push(shell::cd(ancestor.to_str().unwrap()));
            output.push(shell::source(f.to_str().unwrap()));
        };
    }

    // make sure `cd -` goes back to source folder after cd
    output.push(shell::cd(source.to_str().unwrap()));

    output.push(shell::cd(target.to_str().unwrap()));

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
