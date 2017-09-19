imports!();
use client::{DeleteQueryBuilder, Executor};

new_type!(
    Issues
    IssueIdOrKey
    SharedFiles
    SharedFileId
    Attachments
    AttachmentId
);

from!(
    @DeleteQueryBuilder
        -> Issues = "issues"
    @Issues
        => Executor
        => IssueIdOrKey
    @IssueIdOrKey
        => Executor
        -> Attachments = "attachments"
        -> SharedFiles = "sharedFiles"
    @SharedFiles
        -> SharedFileId = "id"
    @SharedFileId
        => Executor
    @Attachments
        => Executor
        => AttachmentId
    @AttachmentId
        => Executor
);

impl_macro!(
    @Issues
        |
        |=> issue_id_or_key -> IssueIdOrKey = issue_id_or_key
        |-> execute
    @IssueIdOrKey
        |=> attachments -> Attachments
        |=> shared_files -> SharedFiles
        |
        |-> execute
    @SharedFiles
        |
        |=> id -> SharedFileId = id
    @SharedFileId
        |
        |-> execute
    @Attachments
        |
        |=> attachment_id -> AttachmentId = attachment_id
        |-> execute
    @AttachmentId
        |
        |-> execute
);
