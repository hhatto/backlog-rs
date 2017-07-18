imports!();
use client::{GetQueryBuilder, Executor};

new_type!(Priorities);

from!(
    @GetQueryBuilder
        -> Priorities = "priorities"
    @Priorities
        => Executor
);

impl_macro!(
    @Priorities
        |
        |-> execute
);
