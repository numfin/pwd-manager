use eyre::Result;
use meilisearch_sdk::{indexes::Index, Client};

pub enum AppIndex {
    PasswordRecords,
}
impl AppIndex {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::PasswordRecords => "password-records",
        }
    }
}

pub struct Meili {
    pub client: Client,
}
impl Meili {
    pub fn index(&self, index: AppIndex) -> Index {
        self.client.index(index.to_string())
    }

    pub async fn prepare(&self) -> Result<()> {
        self.index(AppIndex::PasswordRecords)
            .set_sortable_attributes(&["title"])
            .await?;
        Ok(())
    }
}

impl Default for Meili {
    fn default() -> Self {
        let client = Client::new(
            std::env::var("MEILI_HOST").expect("MEILI_HOST env"),
            std::env::var("MEILI_KEY").expect("MEILI_KEY env"),
        );
        Self { client }
    }
}
