use super::AggregateFrom;
use crate::line_item::{LineItemSummary, Money};
use std::collections::HashMap;

/// Aggregates Spending by Category
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CategoryAggregate<U: Money> {
    pub category_spend: HashMap<String, U>,
}

impl<U: Money> AggregateFrom<U> for CategoryAggregate<U> {
    fn aggregate_from(summary: LineItemSummary<U>) -> Self {
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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::line_item::LineItem;

    use super::*;

    #[test]
    fn simple_aggregate() {
        let items = vec![
            LineItem {
                flow: -12,
                category: "A".to_string(),
                date: NaiveDate::from_yo(2012, 73),
            },
            LineItem {
                flow: 7,
                category: "B".to_string(),
                date: NaiveDate::from_yo(2012, 73),
            },
            LineItem {
                flow: 7,
                category: "A".to_string(),
                date: NaiveDate::from_yo(2012, 73),
            },
        ];
        let summary = LineItemSummary::summary(items, Vec::new());

        let category_spend = vec![("A".to_string(), -5), ("B".to_string(), 7)]
            .into_iter()
            .collect();
        let expected_aggregate = CategoryAggregate { category_spend };

        let test_aggregate = CategoryAggregate::aggregate_from(summary);
        assert_eq!(test_aggregate, expected_aggregate);
    }
}
