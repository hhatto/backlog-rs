imports!();
use client::{DeleteQueryBuilder, Executor};

new_type!(
    Users
    UserId
);

from!(
    @DeleteQueryBuilder
        -> Users = "users"
    @Users
        => Executor
        => UserId
    @UserId
        => Executor
);

impl_macro!(
    @Users
        |
        |=> user_id -> UserId = user_id
        |-> execute
    @UserId
        |
        |-> execute
);
