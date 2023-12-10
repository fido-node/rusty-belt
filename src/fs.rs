pub fn get_config_path() -> String {
    let xdg_home = std::env::var("XDG_CONFIG_HOME").map(|mut s| {
        s.push_str("/rusty-belt");
        s
    });
    let home = std::env::var("HOME").map(|mut s| {
        s.push_str("/.config/rusty-belt");
        s
    });
    let path = xdg_home.or(home).unwrap();
    path
}
