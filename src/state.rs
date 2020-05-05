//! Shared application state.

#[derive(Debug)]
pub(crate) struct State {
    pub db: mongodb::Client,
}

impl State {
    /// Create a new instance of `State`.
    pub(crate) async fn new() -> tide::Result<Self> {
        let mongo = mongodb::Client::with_uri_str("mongodb://localhost:27017/").await?;
        Ok(Self { db: mongo })
    }

    /// Access the mongodb client.
    pub(crate) fn mongo(&self) -> &mongodb::Client {
        &self.db
    }
}
