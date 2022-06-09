use dotenv_codegen::dotenv;

// use serde::Deserialize;
// use config::ConfigError;

// #[derive(Deserialize)]
// pub struct ServerConfig {
//     pub host: String,
//     pub port: i32
// }

// #[derive(Deserialize)]
// pub struct Config {
//     pub server: ServerConfig
// }

// impl Config {
//     pub fn from_env() -> Result<Self, ConfigError> {
//         let mut cfg = config::Config::new();
//         cfg.merge(config::Environtment::new())?;
//         cfg.try_into()
//     }
// }

pub struct ServerConfig {
    pub host: String,
    pub port: String
}

pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config
}

impl Config {
    pub fn from_env(&mut self) -> &mut Config{
        let host_env = dotenv!("SERVER_HOST");
        let port_env = dotenv!("SERVER_PORT");
        self.set_server(host_env.to_string(), port_env.to_string());
        self
    }

    pub fn set_server(&mut self, host_env: String, port_env: String) {
        self.server = ServerConfig {
            host: host_env,
            port: port_env
        }
    }
}
