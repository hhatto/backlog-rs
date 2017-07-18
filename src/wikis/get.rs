imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Wikis
    Count
    Tags
    WikiId
    Attachments
    AttachmentId
    SharedFiles
    History
    Stars
);

from!(
    @GetQueryBuilder
        -> Wikis = "wikis"
    @Wikis
        => Executor
        => WikiId
        -> Count = "count"
        -> Tags = "tags"
    @Count
        => Executor
    @Tags
        => Executor
    @WikiId
        => Executor
        -> Attachments = "attachments"
        -> SharedFiles = "sharedFiles"
        -> History = "history"
        -> Stars = "stars"
    @Attachments
        => Executor
        => AttachmentId
    @AttachmentId
        => Executor
    @SharedFiles
        => Executor
    @History
        => Executor
    @Stars
        => Executor
);

impl_macro!(
    @Wikis
        |=> count -> Count
        |=> tags -> Tags
        |
        |=> wiki_id -> WikiId = wiki_id
        |-> execute
    @Count
        |
        |-> execute
    @Tags
        |
        |-> execute
    @WikiId
        |=> attachments -> Attachments
        |=> shared_files -> SharedFiles
        |=> history -> History
        |=> stars -> Stars
        |
        |-> execute
    @Attachments
        |
        |=> attachment_id -> AttachmentId = attachment_id
        |-> execute
    @AttachmentId
        |
        |-> execute
    @SharedFiles
        |
        |-> execute
    @History
        |
        |-> execute
    @Stars
        |
        |-> execute
);
