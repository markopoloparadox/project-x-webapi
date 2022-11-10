use std::convert::Infallible;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Request, Schema};
use async_graphql_warp::GraphQLResponse;
use serde_json::json;
use warp::filters::BoxedFilter;
use warp::reply::json;
use warp::{Filter, Rejection, Reply};

use crate::schema::{self, CPUListStorage, PriceListStorage};

async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}

pub fn make_routes(
    cpu_list: CPUListStorage,
    price_list: PriceListStorage,
) -> BoxedFilter<(impl Reply,)> {
    // Build Schema
    let schema = schema::build_schema()
        .data(cpu_list)
        .data(price_list)
        .finish();

    let health = warp::path::end().and_then(health);

    let graphql_handler = warp::post().and(warp::path("graphql").and(
        async_graphql_warp::graphql(schema).and_then(
            |(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        ),
    ));

    let graphql_playground = warp::path("playground").map(|| {
        warp::http::Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    // Wire together all the routes.
    health.or(graphql_handler).or(graphql_playground).boxed()
}
