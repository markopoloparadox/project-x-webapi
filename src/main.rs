use std::sync::Arc;

use async_graphql::futures_util::lock::Mutex;
use schema::{CPUDetails, CPUListStorage, PriceDetails, PriceListStorage};

mod database;
mod schema;
mod server;

#[tokio::main]
async fn main() {
    let cpu_list: CPUListStorage = Arc::new(Mutex::new(Vec::new()));
    {
        let mut value = cpu_list.lock().await;
        let a = CPUDetails {
            id: "A".into(),
            manufacturer: "B".to_string(),
            architecture: "C".to_string(),
            family: "D".to_string(),
            model: "E".to_string(),
            launch_price: 23f32,
            release_date: "G".to_string(),
        };
        value.push(a);
    }

    let price_list: PriceListStorage = Arc::new(Mutex::new(Vec::new()));
    {
        let mut value = price_list.lock().await;
        let a = PriceDetails {
            id: "A".into(),
            manufacturer: "B".to_string(),
            model: "E".to_string(),
            price: 23f32,
            date_time: "2022-10-04".to_string(),
            shop: "Links".to_string(),
        };
        value.push(a);
    }

    server::start(([127, 0, 0, 1], 8080), cpu_list, price_list).await;
}
