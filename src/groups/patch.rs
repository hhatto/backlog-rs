imports!();
use client::{PatchQueryBuilder, Executor};

new_type!(
    Groups
    GroupId
);

from!(
    @PatchQueryBuilder
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
