imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Issues
    IssueIdOrKey
    SharedFiles
    IssueCount
    Comments
    CommentId
    CommentsCount
    Attachments
    AttachmentId
    Notifications
);

from!(
    @GetQueryBuilder
        -> Issues = "issues"
    @Issues
        => Executor
        => IssueIdOrKey
        -> IssueCount = "count"
    @IssueIdOrKey
        => Executor
        -> Comments = "comments"
        -> Attachments = "attachments"
        -> SharedFiles = "sharedFiles"
    @Comments
        => Executor
        => CommentId
        -> CommentsCount = "count"
    @SharedFiles
        => Executor
    @CommentId
        => Executor
        -> Notifications = "notifications"
    @Notifications
        => Executor
    @IssueCount
        => Executor
    @CommentsCount
        => Executor
    @Attachments
        => Executor
        => AttachmentId
    @AttachmentId
        => Executor
);

impl_macro!(
    @Issues
        |=> count -> IssueCount
        |
        |=> issue_id_or_key -> IssueIdOrKey = issue_id_or_key
        |-> execute
    @IssueIdOrKey
        |=> comments -> Comments
        |=> attachments -> Attachments
        |=> shared_files -> SharedFiles
        |
        |-> execute
    @SharedFiles
        |
        |-> execute
    @IssueCount
        |
        |-> execute
    @Comments
        |=> count -> CommentsCount
        |
        |=> comment_id -> CommentId = comment_id
        |-> execute
    @CommentsCount
        |
        |-> execute
    @CommentId
        |=> notifications -> Notifications
        |
        |-> execute
    @Notifications
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
