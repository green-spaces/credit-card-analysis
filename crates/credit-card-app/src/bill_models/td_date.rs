use chrono::naive::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TdDate(NaiveDate);

// TryFrom has a conflicting implementation from core (its incorrect though)
impl<S: AsRef<str>> From<S> for TdDate {
    fn from(s: S) -> Self {
        let date = NaiveDate::parse_from_str(s.as_ref(), "%m/%d/%Y").unwrap();
        TdDate(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let date = "07/21/2022";
        let expected_date = TdDate(NaiveDate::from_ymd(2022, 7, 21));
        let parsed_date = TdDate::from(date);
        assert_eq!(parsed_date, expected_date);
    }
}
