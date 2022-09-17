use std::convert::Infallible;

use super::bill_line_string::BillLineString;
use super::td_date::TdDate;

type Money = f32;

#[derive(Debug, Clone, PartialEq)]
pub struct BillLine {
    transaction_data: TdDate,
    description: String,
    debit: Option<Money>,
    credit: Option<Money>,
    balance: Money,
}

impl TryFrom<BillLineString> for BillLine {
    type Error = Infallible;

    fn try_from(v: BillLineString) -> Result<Self, Self::Error> {
        let transaction_data = TdDate::from(v.0[0].clone());
        let description = v.0[1].clone();
        let debit: Option<Money> = v.0[2].clone().parse().ok();
        let credit: Option<Money> = v.0[3].clone().parse().ok();
        let balance: Money = v.0[4].clone().parse().unwrap();

        Ok(BillLine {
            transaction_data,
            description,
            debit,
            credit,
            balance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bill_line_strings() {
        let input = BillLineString(vec![
            "07/21/2022".to_string(),
            "COFFEE ISLAND".to_string(),
            "11.83".to_string(),
            "".to_string(),
            "164.19".to_string(),
        ]);

        let expected_bill_line = BillLine {
            transaction_data: TdDate::from("07/21/2022"),
            description: "COFFEE ISLAND".to_string(),
            debit: Some("11.83".to_string().parse().unwrap()),
            credit: None,
            balance: "164.19".parse().unwrap(),
        };

        let parsed_bill_line: BillLine = input.try_into().unwrap();

        assert_eq!(parsed_bill_line, expected_bill_line);
    }
}
