use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    mcserver: Option<ConfigTomlMcServer>,
    server: Option<ConfigTomlServer>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlMcServer {
    project_name: Option<String>,
    logfile: Option<String>,
    tunnel: Option<String>,
    java: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlServer {
    online_mode: Option<bool>,
    version: Option<String>,
    server_type: Option<String>,
    category: Option<String>,
    providor: Option<String>,
    url: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub project_name: String,
    pub logfile: String,
    pub tunnel: String,
    pub java: String,
    pub online_mode: bool,
    pub version: String,
    pub server_type: String,
    pub category: String,
    pub providor: String,
    pub url: String,
}



impl Config {
    pub fn new(project_name: &str) -> Self {
        let f1 = format!("./{}/config.toml", project_name);
        let f2 = format!("./{}/config.toml", project_name);

        let config_filepaths: [&str; 2] = [&f1, &f2];
        println!("{:?}", config_filepaths);
        let mut content: String = "".to_owned();

        for filepath in &config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml {
                mcserver: None,
                server: None,
            }
        });

        let (project_name, logfile, tunnel, java): (String, String, String, String) =
            match config_toml.mcserver {
                Some(mcserver) => {
                    let mc_project_name: String = mcserver.project_name.unwrap_or_else(|| {
                        println!("Missing field project_name in table mcserver.");
                        "unknown".to_owned()
                    });
                    let mc_logfile: String = mcserver.logfile.unwrap_or_else(|| {
                        println!("Missing field logfile in table mcserver.");
                        "unknown".to_owned()
                    });

                    let mc_tunnel: String = mcserver.tunnel.unwrap_or_else(|| {
                        println!("Missing field tunnel in table mcserver.");
                        "unknown".to_owned()
                    });

                    let mc_java: String = mcserver.java.unwrap_or_else(|| {
                        println!("Missing field java in table mcserver.");
                        "unknown".to_owned()
                    });
                    (mc_project_name, mc_logfile, mc_tunnel, mc_java)
                }
                None => {
                    println!("Missing table mcserver.");
                    (
                        "unknown".to_owned(),
                        "unknown".to_owned(),
                        "unknown".to_owned(),
                        "unknown".to_owned(),
                    )
                }
            };

        let (online_mode, version, server_type, category, providor, url): (
            bool,
            String,
            String,
            String,
            String,
            String,
        ) = match config_toml.server {
            Some(server) => {
                let srv_online_mode: bool = server.online_mode.unwrap_or_else(|| {
                    println!("Missing field online_mode in table server.");
                    false
                });

                let srv_version: String = server.version.unwrap_or_else(|| {
                    println!("Missing field version in table server.");
                    "unknown".to_owned()
                });

                let srv_type: String = server.server_type.unwrap_or_else(|| {
                    println!("Missing field type in table server.");
                    "unknown".to_owned()
                });
                let srv_category: String = server.category.unwrap_or_else(|| {
                    println!("Missing field type in table server.");
                    "unknown".to_owned()
                });
                let srv_providor: String = server.providor.unwrap_or_else(|| {
                    println!("Missing field type in table server.");
                    "unknown".to_owned()
                });

                let srv_url: String = server.url.unwrap_or_else(|| {
                    println!("Missing field url in table server.");
                    "unknown".to_owned()
                });
                (
                    srv_online_mode,
                    srv_version,
                    srv_type,
                    srv_category,
                    srv_providor,
                    srv_url,
                )
            }
            None => {
                println!("Missing table server.");
                (
                    false,
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                    "unknown".to_owned(),
                )
            }
        };

        Config {
            project_name,
            logfile,
            tunnel,
            java,
            online_mode,
            version,
            server_type,
            category,
            providor,
            url,
        }
    }
}