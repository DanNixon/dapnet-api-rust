use crate::{
    Call, Callsign, News, Node, OutgoingCall, OutgoingNews, Rubric, Statistics, Transmitter,
    TransmitterGroup,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> crate::Result<Option<T>> {
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
            Err(crate::Error::ApiError(result.status()))
        }
    }

    async fn get_many<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> crate::Result<Option<Vec<T>>> {
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
            Err(crate::Error::ApiError(result.status()))
        }
    }

    async fn post<T: Serialize + ?Sized>(&self, path: &str, item: &T) -> crate::Result<()> {
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
            Err(crate::Error::ApiError(result.status()))
        }
    }

    pub async fn get_statistics(&self) -> crate::Result<Option<Statistics>> {
        self.get("stats").await
    }

    pub async fn get_calls_by(&self, owner: &str) -> crate::Result<Option<Vec<Call>>> {
        self.get_many(&format!("calls?ownerName={}", owner)).await
    }

    /// Sends a new call/message.
    ///
    /// Example:
    /// ```no_run
    /// # use dapnet_api::{OutgoingCall, Client};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = Client::new("m0nxn", "my_super_secret_password");
    /// client
    ///     .new_call(&OutgoingCall::new(
    ///         "M0NXN: this is a test".to_string(),
    ///         vec!["m0nxn".to_string()],
    ///         vec!["uk-all".to_string()],
    ///     ))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn new_call(&self, call: &OutgoingCall) -> crate::Result<()> {
        self.post("calls", call).await
    }

    pub async fn get_all_nodes(&self) -> crate::Result<Option<Vec<Node>>> {
        self.get_many("nodes").await
    }

    pub async fn get_node(&self, name: &str) -> crate::Result<Option<Node>> {
        self.get(&format!("nodes/{}", name)).await
    }

    pub async fn get_all_callsigns(&self) -> crate::Result<Option<Vec<Callsign>>> {
        self.get_many("callsigns").await
    }

    pub async fn get_callsign(&self, name: &str) -> crate::Result<Option<Callsign>> {
        self.get(&format!("callsigns/{}", name)).await
    }

    pub async fn get_all_transmitters(&self) -> crate::Result<Option<Vec<Transmitter>>> {
        self.get_many("transmitters").await
    }

    pub async fn get_transmitter(&self, name: &str) -> crate::Result<Option<Transmitter>> {
        self.get(&format!("transmitters/{}", name)).await
    }

    pub async fn get_all_transmitter_groups(&self) -> crate::Result<Option<Vec<TransmitterGroup>>> {
        self.get_many("transmitterGroups").await
    }

    pub async fn get_transmitter_group(
        &self,
        name: &str,
    ) -> crate::Result<Option<TransmitterGroup>> {
        self.get(&format!("transmitterGroups/{}", name)).await
    }

    pub async fn get_all_rubrics(&self) -> crate::Result<Option<Vec<Rubric>>> {
        self.get_many("rubrics").await
    }

    pub async fn get_rubric(&self, name: &str) -> crate::Result<Option<Rubric>> {
        self.get(&format!("rubrics/{}", name)).await
    }

    pub async fn get_news(&self, name: &str) -> crate::Result<Option<Vec<News>>> {
        match self
            .get_many::<Option<News>>(&format!("news?rubricName={}", name))
            .await?
        {
            Some(v) => Ok(Some(v.into_iter().flatten().collect())),
            None => Ok(None),
        }
    }

    /// Sends news to a rubric.
    ///
    /// Example:
    /// ```no_run
    /// # use dapnet_api::{Client, OutgoingNews};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = Client::new("m0nxn", "my_super_secret_password");
    /// client
    ///     .new_news(&OutgoingNews::new(
    ///         "some_rubric_name".to_string(),
    ///         "M0NXN: this is a test".to_string(),
    ///     ))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn new_news(&self, news: &OutgoingNews) -> crate::Result<()> {
        self.post("news", news).await
    }
}
