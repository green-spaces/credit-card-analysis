pub mod filter;

use chrono::NaiveDate;
use std::ops::{Add, AddAssign, Neg};

/// Any object with these traits can be used as money
pub trait Money: Add<Output = Self> + AddAssign + Neg<Output = Self> + Ord + Copy {}

impl Money for i8 {}
impl Money for i16 {}
impl Money for i32 {}
impl Money for i64 {}

#[derive(Debug, Clone)]
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
    pub fn summary<F: Fn(&LineItem<U>) -> bool>(items: Vec<LineItem<U>>, filters: Vec<F>) -> Self {
        let filtered_items = items
            .into_iter()
            .filter(|item| {
                let res = filters.iter().all(|filter| filter(item));
                res
            })
            .collect();
        Self {
            items: filtered_items,
        }
    }
}
