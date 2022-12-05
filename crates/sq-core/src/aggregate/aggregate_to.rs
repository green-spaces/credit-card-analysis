use crate::line_item::{LineItemReduction, Money};

/// Aggregates a summary
pub trait AggregateFrom<U: Money> {
    fn aggregate_from(summary: LineItemReduction<U>) -> Self;
}
