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


    // if let Some(f) = get_envleave_file(&source) {
    //     output.push(shell_command::source(f.to_str().unwrap()));
    // }

    output.push(shell_command::cd(target.to_str().unwrap()));

    // if let Some(f) = get_env_file(&target) {
    //     output.push(shell_command::source(f.to_str().unwrap()));
    // }

    output.send()
}


fn get_envleave_file(dir: &Path) -> Option<PathBuf> {
    todo!()
}


fn get_env_file(dir: &Path) -> Option<PathBuf> {
    todo!()
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

// fn get_env_file(cwd: PathBuf) -> Result<PathBuf, String> {
//     let p = Path::join(&self, path)
// }
