imports!();
use client::{PatchQueryBuilder, Executor};

new_type!(
    Issues
    IssueIdOrKey
    Comments
    CommentId
);

from!(
    @PatchQueryBuilder
        -> Issues = "issues"
    @Issues
        => IssueIdOrKey
    @IssueIdOrKey
        => Executor
        -> Comments = "comments"
    @Comments
        => CommentId
    @CommentId
        => Executor
);

impl_macro!(
    @Issues
        |
        |=> issue_id_or_key -> IssueIdOrKey = issue_id_or_key
    @IssueIdOrKey
        |=> comments -> Comments
        |
        |-> execute
    @Comments
        |
        |=> comment_id -> CommentId = comment_id
    @CommentId
        |
        |-> execute
);
