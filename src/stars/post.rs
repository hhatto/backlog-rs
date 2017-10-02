imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Stars
);

from!(
    @PostQueryBuilder
        -> Stars = "stars"
    @Stars
        => Executor
);

impl_macro!(
    @Stars
        |
        |-> execute
);
