use crate::line_item::{LineItemSummary, Money};

pub trait AggregateTo<A, U: Money> {
    fn aggregate_to(summary: LineItemSummary<U>) -> A;
}
