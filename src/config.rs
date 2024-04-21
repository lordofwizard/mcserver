use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    mcserver: McServerConfig,
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct McServerConfig {
    logfile: String,
    tunnel: String,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    online_mode: bool,
    version: String,
    r#type: String, // type is a reserved keyword in Rust, so use r#type
    url: String,
}

pub fn take_config() {
    // Read the contents of the file into a string
    let mut file = File::open("config.toml").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    // Deserialize the TOML into a Config struct
    let config: Config = toml::from_str(&contents).expect("Unable to parse TOML");

    // Access values
    let mcserver_logfile = config.mcserver.logfile;
    let mcserver_tunnel = config.mcserver.tunnel;

    let server_online_mode = config.server.online_mode;
    let server_version = config.server.version;
    let server_type = config.server.r#type; // using r#type to avoid the reserved keyword
    let server_url = config.server.url;

    // Print values or use them in your program
    println!("MCServer Logfile: {}", mcserver_logfile);
    println!("MCServer Tunnel: {}", mcserver_tunnel);

    println!("Server Online Mode: {}", server_online_mode);
    println!("Server Version: {}", server_version);
    println!("Server Type: {}", server_type);
    println!("Server URL: {}", server_url);
}
