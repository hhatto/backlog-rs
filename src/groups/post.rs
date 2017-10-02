imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Groups
);

from!(
    @PostQueryBuilder
        -> Groups = "groups"
    @Groups
        => Executor
);

impl_macro!(
    @Groups
        |
        |-> execute
);
