imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Wikis
    WikiId
    Attachments
    SharedFiles
);

from!(
    @PostQueryBuilder
        -> Wikis = "wikis"
    @Wikis
        => Executor
        => WikiId
    @WikiId
        => Executor
        -> Attachments = "attachments"
        -> SharedFiles = "sharedFiles"
    @Attachments
        => Executor
    @SharedFiles
        => Executor
);

impl_macro!(
    @Wikis
        |
        |=> wiki_id -> WikiId = wiki_id
    @WikiId
        |=> attachments -> Attachments
        |=> shared_files -> SharedFiles
        |
    @Attachments
        |
        |-> execute
    @SharedFiles
        |
        |-> execute
);
