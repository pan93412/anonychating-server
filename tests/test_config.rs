use anonychating_server::config;

#[test]
fn test_config() {
    let c = config::Config::from_file("config.example.toml");
    assert_eq!(c.telexide.bot_token, "");
    assert_eq!(c.telexide.publish_channel, "@");
    assert_eq!(c.warp.server_ip, [127, 0, 0, 1]);
    assert_eq!(c.warp.server_port, 3030);
}