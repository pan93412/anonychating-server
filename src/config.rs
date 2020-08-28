use serde::Deserialize;
use std::fs;

/// The IP format.
type IPv4 = [u8; 4];

/// The configuration of anonychating_server.
#[derive(Deserialize)]
pub struct Config {
    /// The configuration struct of Telexide.
    pub telexide: TelexideConfig,

    /// The configuration struct of Warp.
    pub warp: WarpConfig,
}

impl Config {
    pub fn from_file(filename: &str) -> Self {
        let errmsg_of_read_to_string: &str =
            &format!("I can't find the configuration file: {}", filename);

        toml::from_str(&fs::read_to_string(filename).expect(errmsg_of_read_to_string))
            .expect("The configuration file is invaild.")
    }
}

/// The configuration struct of Telexide.
#[derive(Deserialize)]
pub struct TelexideConfig {
    /// The token of your Telegram bot.
    pub bot_token: String,

    /// The channel to publish the received message.
    pub publish_channel: String,
}

/// The configuration struct of Warp.
#[derive(Deserialize)]
pub struct WarpConfig {
    /// The IP assocated to your server.
    pub server_ip: IPv4,
    /// The port the user specified.
    pub server_port: u16,
}

impl WarpConfig {
    pub fn to_uri(&self) -> String {
        format!(
            "{}:{}",
            self.server_ip.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("."),
            self.server_port,
        )
    }
}
