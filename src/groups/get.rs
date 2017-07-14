imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Groups
    GroupId
);

from!(
    @GetQueryBuilder
        -> Groups = "groups"
    @Groups
        => Executor
        => GroupId
    @GroupId
        => Executor
);

impl_macro!(
    @Groups
        |
        |=> group_id -> GroupId = group_id
        |-> execute
    @GroupId
        |
        |-> execute
);
