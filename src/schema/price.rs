use std::sync::Arc;

use async_graphql::futures_util::lock::Mutex;
use async_graphql::{Object, ID};

#[derive(Clone)]
pub struct PriceDetails {
    pub id: ID,
    pub manufacturer: String,
    pub model: String,
    pub price: f32,
    pub date_time: String,
    pub shop: String,
}

#[Object]
impl PriceDetails {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    async fn model(&self) -> &str {
        &self.model
    }

    async fn launch_price(&self) -> f32 {
        self.price
    }

    async fn date_time(&self) -> &str {
        &self.date_time
    }

    async fn shop(&self) -> &str {
        &self.shop
    }
}

pub type PriceListStorage = Arc<Mutex<Vec<PriceDetails>>>;
