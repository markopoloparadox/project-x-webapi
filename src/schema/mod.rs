mod cpu;
mod price;
mod root;

pub use cpu::{CPUDetails, CPUListStorage};
pub use price::{PriceDetails, PriceListStorage};

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(root::RootQuery);

pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
