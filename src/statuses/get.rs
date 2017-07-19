imports!();
use client::{GetQueryBuilder, Executor};

new_type!(Statuses);

from!(
    @GetQueryBuilder
        -> Statuses = "statuses"
    @Statuses
        => Executor
);

impl_macro!(
    @Statuses
        |
        |-> execute
);
