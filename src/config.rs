use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThingputConfig {
    pub http: HttpConfig,
}

#[derive(Deserialize)]
pub struct HttpConfig {
    address: String,
    port: u16,
}

impl ThingputConfig {
    // Empty constructor with default values
    pub fn new() -> ThingputConfig {
        ThingputConfig {
            http: HttpConfig {
                address: String::from("0.0.0.0"),
                port: 8080,
            },
        }
    }
}

impl HttpConfig {
    pub fn get_url(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
