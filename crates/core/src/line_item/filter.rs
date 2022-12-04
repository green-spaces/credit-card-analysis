use chrono::NaiveDate;

use super::{LineItem, Money};

pub struct LineFilter<U: Money> {
    filter_func: Box<dyn Fn(&LineItem<U>) -> bool>,
}

impl<U: Money> LineFilter<U> {
    /// Allows for the creation of new filters
    pub fn new(filter: Box<dyn Fn(&LineItem<U>) -> bool>) -> Self {
        Self {
            filter_func: filter,
        }
    }

    /// Applies the filter
    pub fn apply(&self, line_item: &LineItem<U>) -> bool {
        (self.filter_func)(line_item)
    }

    /// Produces a function that returns true when the [LineItem]'s data is on is on or after the supplied date
    pub fn item_date_on_or_after(start: NaiveDate) -> Self {
        Self {
            filter_func: Box::new(move |item: &LineItem<U>| item.date >= start),
        }
    }

    /// Produces a function that returns true when the [LineItem]'s data is before the
    pub fn item_date_before(end: NaiveDate) -> Self {
        Self {
            filter_func: Box::new(move |item: &LineItem<U>| item.date < end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    mod date_on_or_after {
        use super::*;

        #[test]
        fn filter_date_before_item_date() {
            let item_date = NaiveDate::from_ymd(2022, 1, 1);
            let filter_date = item_date - Duration::days(1);
            let item = LineItem {
                flow: 0,
                category: "Category".to_string(),
                date: item_date,
            };

            // Test
            let lf = LineFilter::item_date_on_or_after(filter_date);
            assert!(lf.apply(&item))
        }

        #[test]
        fn filter_date_on_item_date() {
            let item_date = NaiveDate::from_ymd(2022, 1, 1);
            let start_after_item = item_date;
            let item = LineItem {
                flow: 0,
                category: "Category".to_string(),
                date: item_date,
            };

            let lf = LineFilter::item_date_on_or_after(start_after_item);
            assert!(lf.apply(&item))
        }

        #[test]
        fn filter_date_after_item_date() {
            let item_date = NaiveDate::from_ymd(2022, 1, 1);
            let start_after_item = item_date + Duration::days(1);
            let item = LineItem {
                flow: 0,
                category: "Category".to_string(),
                date: item_date,
            };

            let lf = LineFilter::item_date_on_or_after(start_after_item);
            assert!(!lf.apply(&item))
        }
    }

    mod date_before {
        use super::*;

        #[test]
        fn filter_date_before_item_date() {
            let item_date = NaiveDate::from_ymd(2022, 1, 1);
            let start_after_item = item_date - Duration::days(1);
            let item = LineItem {
                flow: 0,
                category: "Category".to_string(),
                date: item_date,
            };

            // Test
            let lf = LineFilter::item_date_before(start_after_item);
            assert!(!lf.apply(&item))
        }

        #[test]
        fn filter_date_on_item_date() {
            let item_date = NaiveDate::from_ymd(2022, 1, 1);
            let start_after_item = item_date;
            let item = LineItem {
                flow: 0,
                category: "Category".to_string(),
                date: item_date,
            };

            let lf = LineFilter::item_date_before(start_after_item);
            assert!(!lf.apply(&item))
        }

        #[test]
        fn filter_date_after_item_date() {
            let item_date = NaiveDate::from_ymd(2022, 1, 1);
            let start_after_item = item_date + Duration::days(1);
            let item = LineItem {
                flow: 0,
                category: "Category".to_string(),
                date: item_date,
            };

            let lf = LineFilter::item_date_before(start_after_item);
            assert!(lf.apply(&item))
        }
    }
}
