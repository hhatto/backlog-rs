imports!();
use client::{DeleteQueryBuilder, Executor};

new_type!(
    Groups
    GroupId
);

from!(
    @DeleteQueryBuilder
        -> Groups = "groups"
    @Groups
        => GroupId
    @GroupId
        => Executor
);

impl_macro!(
    @Groups
        |
        |=> group_id -> GroupId = group_id
    @GroupId
        |
        |-> execute
);
