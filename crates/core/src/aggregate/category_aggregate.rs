use super::AggregateTo;
use crate::line_item::{LineItemSummary, Money};
use std::collections::HashMap;

pub struct CategoryAggregate<U: Money> {
    pub category_spend: HashMap<String, U>,
}

impl<U: Money> AggregateTo<CategoryAggregate<U>, U> for LineItemSummary<U> {
    fn aggregate_to(summary: LineItemSummary<U>) -> CategoryAggregate<U> {
        let mut spending = HashMap::new();
        summary.items.iter().for_each(|item| {
            spending
                .entry(item.category.clone())
                .and_modify(|e| *e += item.flow)
                .or_insert(item.flow);
        });

        CategoryAggregate {
            category_spend: spending,
        }
    }
}
