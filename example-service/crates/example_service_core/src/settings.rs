use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoreSettings {
    clients: HashMap<String, ClientConfig>,
}

impl CoreSettings {
    pub fn new() -> CoreSettings {
        CoreSettings {
            clients: Default::default(),
        }
    }

    pub fn clients(&self) -> &HashMap<String, ClientConfig> {
        &self.clients
    }

    pub fn clients_mut(&mut self) -> &mut HashMap<String, ClientConfig> {
        &mut self.clients
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClientConfig {
    endpoint: String,
}

impl ClientConfig {
    pub fn new<T: Into<String>>(endpoint: T) -> ClientConfig {
        ClientConfig {
            endpoint: endpoint.into(),
        }
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
}

impl Default for CoreSettings {
    fn default() -> Self {
        // Put client defaults here
        let client_map = HashMap::new();

        CoreSettings { clients: client_map }
    }
}