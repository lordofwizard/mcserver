use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::env;

#[derive(Deserialize, Serialize)]
struct Server {
    r#type: String,
}

#[derive(Deserialize, Serialize)]
struct Settings {
    server: Server,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            server: Server {
                r#type: "java".to_string(),
            },
        }
    }
}

pub fn take_config() -> Result<(), ConfyError> {
    let mut path = env::current_dir().unwrap();
    path.push("config.toml");
    let cfg: Settings = confy::load_path(path).unwrap();
    println!("Server type: {}", cfg.server.r#type);
    Ok(())
}