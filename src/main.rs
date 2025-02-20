use std::path::{Path, PathBuf};
use std::env::{home_dir, current_dir}; 

fn main() {
    let args = std::env::args();
    let cwd = current_dir().unwrap();

    let dir_arg = args.last()
                      .unwrap_or(home_dir().unwrap().into_os_string().into_string().unwrap());
    let dir = PathBuf::from(dir_arg);


}


fn get_env_file(cwd: PathBuf) -> Result<PathBuf, String> {
    let p = Path::join(&self, path)
}
