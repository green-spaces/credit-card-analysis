use crate::line_item::{LineItemSummary, Money};

/// Aggregates a summary
pub trait AggregateFrom<U: Money> {
    fn aggregate_from(summary: LineItemSummary<U>) -> Self;
}
