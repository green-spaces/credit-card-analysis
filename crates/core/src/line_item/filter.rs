use chrono::NaiveDate;

use super::{LineItem, Money};

/// Produces a function that returns true when the [LineItem]'s data is on is on or after the supplied date
pub fn date_on_or_after<U: Money>(start: NaiveDate) -> impl Fn(&LineItem<U>) -> bool {
    move |line: &LineItem<U>| line.date >= start
}

/// Produces a function that returns true when the [LineItem]'s data is before the
pub fn date_before<U: Money>(end: NaiveDate) -> impl Fn(&LineItem<U>) -> bool {
    move |line: &LineItem<U>| line.date < end
}
