mod filter;

use chrono::NaiveDate;
use std::ops::{Add, AddAssign, Neg};

pub use filter::LineFilter;

/// Any object with these traits can be used as money
pub trait Money: Add<Output = Self> + AddAssign + Neg<Output = Self> + Ord + Copy {}

impl Money for i8 {}
impl Money for i16 {}
impl Money for i32 {}
impl Money for i64 {}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a single transaction
///
/// U > 0 is assumed to be money spend, U < 0 is money received
pub struct LineItem<U: Money> {
    pub flow: U,
    pub category: String,
    pub date: NaiveDate,
}

/// Contains a bunch of LineItems that can then be aggregated together
pub struct LineItemSummary<U: Money> {
    pub items: Vec<LineItem<U>>,
}

impl<U: Money> LineItemSummary<U> {
    /// Creates a summary object from a vector of LineItems and filter functions.
    ///
    /// All filters must be true for the line to be included
    pub fn summary(items: Vec<LineItem<U>>, filters: Vec<LineFilter<U>>) -> Self {
        let filtered_items = items
            .into_iter()
            .filter(|item| {
                let res = filters.iter().all(|filter| filter.call(item));
                res
            })
            .collect();
        Self {
            items: filtered_items,
        }
    }
}

mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn empty_line_item_summary() {
        let summary: LineItemSummary<i32> = LineItemSummary::summary(Vec::new(), Vec::new());
        assert!(summary.items.is_empty());
    }

    #[test]
    fn line_item_filtered_summary() {
        let line_items = vec![LineItem {
            flow: 1,
            category: "Hello".to_string(),
            date: NaiveDate::from_ymd(2, 1, 1),
        }];
        let filters = vec![LineFilter::item_date_before(NaiveDate::from_ymd(1, 1, 1))];
        let summary: LineItemSummary<i32> = LineItemSummary::summary(line_items, filters);
        assert!(summary.items.is_empty());
    }

    #[test]
    fn line_item_not_filtered_summary() {
        let line_items = vec![LineItem {
            flow: 1,
            category: "Hello".to_string(),
            date: NaiveDate::from_ymd(2021, 1, 1),
        }];
        let filters = vec![
            LineFilter::item_date_on_or_after(NaiveDate::from_ymd(2020, 1, 1)),
            LineFilter::item_date_before(NaiveDate::from_ymd(2023, 1, 1)),
        ];

        let summary: LineItemSummary<i32> = LineItemSummary::summary(line_items.clone(), filters);
        assert_eq!(summary.items, line_items);
    }
}
