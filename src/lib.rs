pub mod error;
pub mod generation;
#[cfg(feature = "chat-history")]
pub mod history;
pub mod models;

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) host: String,
    pub(crate) port: Option<u16>,
    pub(crate) reqwest_client: reqwest::Client,
    #[cfg(feature = "chat-history")]
    pub(crate) messages_history: Option<history::MessagesHistory>,
}

impl Ollama {
    pub fn new(host: String) -> Self {
        Self {
            host,
            ..Default::default()
        }
    }

    pub fn new_with_client(host: String, reqwest_client: reqwest::Client) -> Self {
        Self {
            host,
            reqwest_client,
            ..Default::default()
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_default_port(mut self) -> Self {
        self.port = Some(11434);
        self
    }

    /// Returns the http URI of the Ollama instance
    pub fn uri(&self) -> String {
        match self.port {
            Some(port) => format!("{}:{}", self.host, port),
            None => self.host.clone(),
        }
    }
}

impl Default for Ollama {
    /// Returns a default Ollama instance with the host set to `http://127.0.0.1:11434`.
    fn default() -> Self {
        Self {
            host: "http://127.0.0.1".to_string(),
            port: None,
            reqwest_client: reqwest::Client::new(),
            #[cfg(feature = "chat-history")]
            messages_history: None,
        }
    }
}
