use chrono::naive::NaiveDate;

enum EntryLineType {
    Credit,
    Debit,
}

enum Description {
    With(String),
    Without(String),
}

// Can be debit or credit
pub struct EntryLine {
    // Description on a single line
    description: String,
    // Multiple descriptions may map to a single category
    category: Option<String>,
    entry_type: EntryLineType,
    // in cents
    value: u32,
    // in cents
    balance: u32,
    date: NaiveDate,
}
