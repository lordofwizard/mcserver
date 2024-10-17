use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    mcserver: McServer,
    server: Server,
}

#[derive(Serialize, Deserialize, Debug)]
struct McServer {
    project_name: String,
    logfile: String,
    tunnel: String,
    java: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    online_mode: bool,
    version: String,
    server_type: String,
    category: String,
    providor: String,
}