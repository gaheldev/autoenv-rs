pub fn source(f: &str) -> String {
    format!("source \"{f}\"")
}

// TODO: parameterize cd?
pub fn cd(dir: &str) -> String {
    format!("builtin cd \"{dir}\"")
}

pub fn verbose(f: &str) -> String {
    let text = format!("source {f}");
    let dashes = "-".repeat(text.len());
    format!("echo source {f}; echo {dashes}")
}

// TODO: ask for authorization, if yes source and add to authorized
// ~/.config/autoenv-rs/authorized -> authorized env files
// ~/.cache/autoenv-rs/hashes -> their hashes in same order

// TODO: is_authorized(env: &str) -> bool

// TODO: source only if authorized and hash is same
