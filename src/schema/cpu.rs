use std::sync::Arc;

use async_graphql::futures_util::lock::Mutex;
use async_graphql::{InputObject, Object, ID};

#[derive(Clone, InputObject)]
pub struct CPUDetails {
    pub id: ID,
    pub manufacturer: String,
    pub architecture: String,
    pub family: String,
    pub model: String,
    pub launch_price: f32,
    pub release_date: String,
}

#[Object]
impl CPUDetails {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    async fn architecture(&self) -> &str {
        &self.architecture
    }

    async fn family(&self) -> &str {
        &self.family
    }

    async fn model(&self) -> &str {
        &self.model
    }

    async fn launch_price(&self) -> f32 {
        self.launch_price
    }

    async fn release_date(&self) -> &str {
        &self.release_date
    }
}

pub type CPUListStorage = Arc<Mutex<Vec<CPUDetails>>>;
