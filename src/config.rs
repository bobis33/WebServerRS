pub struct Config {
    pub api_scope: &'static str,
    pub host: &'static str,
    pub port: u16,
    pub db_url: &'static str,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            api_scope: "/v1",
            host: "0.0.0.0",
            port: 8080,
            db_url: "postgres://user:password@service_postgresql:8000/db",
        }
    }
}

pub const CONFIG: Config = Config::new();
