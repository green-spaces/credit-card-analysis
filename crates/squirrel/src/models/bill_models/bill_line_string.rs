/// A single bill line that has been split by the delimiter
pub struct BillLineString(pub Vec<String>);

impl BillLineString {
    pub fn parse_csv(s: String) -> Vec<Self> {
        s.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(|line| {
                Self(
                    line.trim()
                        .split(',')
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csv() {
        let csv = r#"
				04/15/2022,FARMS,688.69,,888.10
				01/16/2022,DRUG MART,21.80,,100.90

			"#;
        let bill_line_strings = BillLineString::parse_csv(csv.to_string());
        assert_eq!(bill_line_strings.len(), 2);
    }
}
