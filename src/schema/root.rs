use async_graphql::{ComplexObject, Context, Object, SimpleObject};

use super::cpu::CPUDetails;
use super::{CPUListStorage, PriceDetails, PriceListStorage};

#[derive(Default)]
pub struct RootQuery();

#[Object]
impl RootQuery {
    async fn price_list(&self, ctx: &Context<'_>) -> Vec<PriceDetails> {
        let value = ctx.data::<PriceListStorage>().unwrap().lock().await;
        value.clone()
    }

    async fn cpu_list(&self, ctx: &Context<'_>) -> Vec<CPUDetails> {
        let value = ctx.data::<CPUListStorage>().unwrap().lock().await;
        value.clone()
    }
}
