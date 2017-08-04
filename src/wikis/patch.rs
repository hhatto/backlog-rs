imports!();
use client::{PatchQueryBuilder, Executor};

new_type!(
    Wikis
    WikiId
);

from!(
    @PatchQueryBuilder
        -> Wikis = "wikis"
    @Wikis
        => WikiId
    @WikiId
        => Executor
);

impl_macro!(
    @Wikis
        |
        |=> wiki_id -> WikiId = wiki_id
    @WikiId
        |
        |-> execute
);
