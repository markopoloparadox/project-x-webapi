use std::net::SocketAddr;

use crate::schema::{CPUListStorage, PriceListStorage};

pub mod routes;

pub async fn start(
    addr: impl Into<SocketAddr>,
    cpu_list: CPUListStorage,
    price_list: PriceListStorage,
) {
    warp::serve(routes::make_routes(cpu_list, price_list))
        .run(addr)
        .await;
}
