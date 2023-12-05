use std::{
    process::Command,
    str::{from_utf8_mut, Utf8Error},
};

pub fn fetch_tmux_name() -> Result<String, Utf8Error> {
    let mut tmux_session = Command::new("sh")
        .arg("-c")
        .arg("tmux display-message -p '#S'")
        .output()
        .expect("failed to execute process")
        .stdout;

    from_utf8_mut(tmux_session.as_mut_slice()).map(|s| s.trim().to_string())
}
// tmux display-message -p '#{pane_current_path}'

pub fn fetch_tmux_current_path() -> Result<String, Utf8Error> {
    let mut tmux_session = Command::new("sh")
        .arg("-c")
        .arg("tmux display-message -p '#{pane_current_path}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    from_utf8_mut(tmux_session.as_mut_slice()).map(|s| s.trim().to_string())
}
