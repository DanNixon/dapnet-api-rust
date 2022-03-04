use crate::{Call, Callsign, Node, Rubric, Statistics, Transmitter, TransmitterGroup};
use anyhow::{anyhow, Result};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ClientConfig {
    pub api_url: Url,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_url: Url::parse("https://hampager.de/api/").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    username: String,
    password: String,
    config: ClientConfig,
}

impl Client {
    /// Creates a new instance of the client with the default configuration.
    ///
    /// Example:
    /// ```
    /// use dapnet_api::Client;
    /// let client = Client::new("m0nxn", "my_super_secret_password");
    /// ```
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            username: username.to_string(),
            password: password.to_string(),
            config: ClientConfig::default(),
        }
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<Option<T>> {
        let result = self
            .client
            .get(self.config.api_url.join(path)?)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if result.status().is_success() {
            Ok(Some(result.json().await?))
        } else if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            Err(anyhow! {"API error: {}", result.status()})
        }
    }

    async fn get_many<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<Option<Vec<T>>> {
        let result = self
            .client
            .get(self.config.api_url.join(path)?)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if result.status().is_success() {
            Ok(Some(result.json().await?))
        } else if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            Err(anyhow! {"API error: {}", result.status()})
        }
    }

    async fn post<T: Serialize + ?Sized>(&self, path: &str, item: &T) -> Result<()> {
        let result = self
            .client
            .post(self.config.api_url.join(path)?)
            .basic_auth(&self.username, Some(&self.password))
            .json(item)
            .send()
            .await?;

        if result.status().is_success() {
            Ok(())
        } else {
            Err(anyhow! {"API error: {}", result.status()})
        }
    }

    pub async fn get_statistics(&self) -> Result<Option<Statistics>> {
        Ok(self.get("stats").await?)
    }

    pub async fn get_calls_by(&self, owner: &str) -> Result<Option<Vec<Call>>> {
        Ok(self.get_many(&format!("calls?ownerName={}", owner)).await?)
    }

    /// Sends a new call/message.
    ///
    /// Example:
    /// ```no_run
    /// # use dapnet_api::{Call, Client};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = Client::new("m0nxn", "my_super_secret_password");
    /// client
    ///     .new_call(&Call::new(
    ///         "M0NXN: this is a test".to_string(),
    ///         vec!["m0nxn".to_string()],
    ///         vec!["uk-all".to_string()],
    ///     ))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn new_call(&self, call: &Call) -> Result<()> {
        self.post("calls", call).await
    }

    pub async fn get_all_nodes(&self) -> Result<Option<Vec<Node>>> {
        Ok(self.get_many("nodes").await?)
    }

    pub async fn get_node(&self, name: &str) -> Result<Option<Node>> {
        Ok(self.get(&format!("nodes/{}", name)).await?)
    }

    pub async fn get_all_callsigns(&self) -> Result<Option<Vec<Callsign>>> {
        Ok(self.get_many("callsigns").await?)
    }

    pub async fn get_callsign(&self, name: &str) -> Result<Option<Callsign>> {
        Ok(self.get(&format!("callsigns/{}", name)).await?)
    }

    pub async fn get_all_transmitters(&self) -> Result<Option<Vec<Transmitter>>> {
        Ok(self.get_many("transmitters").await?)
    }

    pub async fn get_transmitter(&self, name: &str) -> Result<Option<Transmitter>> {
        Ok(self.get(&format!("transmitters/{}", name)).await?)
    }

    pub async fn get_all_transmitter_groups(&self) -> Result<Option<Vec<TransmitterGroup>>> {
        Ok(self.get_many("transmitterGroups").await?)
    }

    pub async fn get_transmitter_group(&self, name: &str) -> Result<Option<TransmitterGroup>> {
        Ok(self.get(&format!("transmitterGroup/{}", name)).await?)
    }

    pub async fn get_all_rubrics(&self) -> Result<Option<Vec<Rubric>>> {
        Ok(self.get_many("rubrics").await?)
    }

    pub async fn get_rubric(&self, name: &str) -> Result<Option<Rubric>> {
        Ok(self.get(&format!("rubrics/{}", name)).await?)
    }

    pub async fn get_news(&self, _rubric: Option<Rubric>) {
        // TODO
    }
}
