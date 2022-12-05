mod filter;

use chrono::NaiveDate;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Neg},
};

pub use filter::LineFilter;

/// Any object with these traits can be used as money
pub trait Money:
    Add<Output = Self>
    + AddAssign
    + Neg<Output = Self>
    + Ord
    + Copy
    + Display
    + std::iter::Sum
    + std::ops::Div
    + TryInto<f64>
{
}

impl Money for i8 {}
impl Money for i16 {}
impl Money for i32 {}
// impl Money for i64 {}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a single transaction
///
/// U > 0 is assumed to be money spend, U < 0 is money received
pub struct LineItem<U: Money> {
    pub flow: U,
    pub category: String,
    pub date: NaiveDate,
}

impl<U: Money> LineItem<U> {
    pub fn new<C: AsRef<str>>(flow: U, category: C, date: NaiveDate) -> Self {
        Self {
            flow,
            category: category.as_ref().to_string(),
            date,
        }
    }
}

/// Contains a reduced of LineItems that can then be aggregated together
pub struct LineItemReduction<U: Money> {
    items: Vec<LineItem<U>>,
}

impl<U: Money> LineItemReduction<U> {
    pub fn items(&self) -> &[LineItem<U>] {
        &self.items
    }
}

impl<U: Money> LineItemReduction<U> {
    /// Creates a summary object from a vector of LineItems and filter functions.
    ///
    /// All filters must be true for the line to be included
    pub fn reduce(items: Vec<LineItem<U>>, filters: Vec<LineFilter<U>>) -> Self {
        let filtered_items = items
            .into_iter()
            .filter(|item| {
                let res = filters.iter().all(|filter| filter.apply(item));
                res
            })
            .collect();
        Self {
            items: filtered_items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line_item_summary() {
        let summary: LineItemReduction<i32> = LineItemReduction::reduce(Vec::new(), Vec::new());
        assert!(summary.items.is_empty());
    }

    #[test]
    fn line_item_filtered_summary() {
        let line_items = vec![LineItem::new(1, "Hello", NaiveDate::from_ymd(2, 1, 1))];
        let filters = vec![LineFilter::item_date_before(NaiveDate::from_ymd(1, 1, 1))];
        let summary: LineItemReduction<i32> = LineItemReduction::reduce(line_items, filters);
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

        let summary: LineItemReduction<i32> =
            LineItemReduction::reduce(line_items.clone(), filters);
        assert_eq!(summary.items, line_items);
    }
}
