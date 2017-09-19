imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Issues
    IssueIdOrKey
    SharedFiles
    Comments
    CommentId
    Notifications
);

from!(
    @PostQueryBuilder
        -> Issues = "issues"
    @Issues
        => Executor
        => IssueIdOrKey
    @IssueIdOrKey
        -> Comments = "comments"
        -> SharedFiles = "sharedFiles"
    @Comments
        => Executor
        => CommentId
    @SharedFiles
        => Executor
    @CommentId
        -> Notifications = "notifications"
    @Notifications
        => Executor
);

impl_macro!(
    @Issues
        |
        |=> issue_id_or_key -> IssueIdOrKey = issue_id_or_key
        |-> execute
    @IssueIdOrKey
        |=> comments -> Comments
        |=> shared_files -> SharedFiles
        |
    @SharedFiles
        |
        |-> execute
    @Comments
        |
        |=> comment_id -> CommentId = comment_id
        |-> execute
    @CommentId
        |=> notifications -> Notifications
        |
    @Notifications
        |
        |-> execute
);
