imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Users
);

from!(
    @PostQueryBuilder
        -> Users = "users"
    @Users
        => Executor
);

impl_macro!(
    @Users
        |
        |-> execute
);
