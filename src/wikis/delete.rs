imports!();
use client::{DeleteQueryBuilder, Executor};

new_type!(
    Wikis
    WikiId
    Attachments
    AttachmentId
    SharedFiles
    SharedFileId
);

from!(
    @DeleteQueryBuilder
        -> Wikis = "wikis"
    @Wikis
        => Executor
        => WikiId
    @WikiId
        => Executor
        -> Attachments = "attachments"
        -> SharedFiles = "sharedFiles"
    @Attachments
        => AttachmentId
    @AttachmentId
        => Executor
    @SharedFiles
        => SharedFileId
    @SharedFileId
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
        |-> execute
    @Attachments
        |
        |=> attachment_id -> AttachmentId = attachment_id
    @AttachmentId
        |
        |-> execute
    @SharedFiles
        |
        |=> shared_file_id -> SharedFileId = shared_file_id
    @SharedFileId
        |
        |-> execute
);
