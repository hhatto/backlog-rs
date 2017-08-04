imports!();
use client::{PatchQueryBuilder, Executor};

new_type!(
    Projects
    ProjectIdOrKey
    IssueTypes
    Categories
    Versions
    CustomFields
    CustomFieldId
    Id
    Webhooks
    WebhookId
    Git
    Repositories
    RepoIdOrName
    PullRequests
    Number
    Comments
    CommentId
);

from!(
    @PatchQueryBuilder
        -> Projects = "projects"
    @Projects
        => ProjectIdOrKey
    @ProjectIdOrKey
        => Executor
        -> IssueTypes = "issueTypes"
        -> Categories = "categories"
        -> Versions = "versions"
        -> CustomFields = "customFields"
        -> Webhooks = "webhooks"
        -> Git = "git"
    @Id
        => Executor
    @IssueTypes
        => Id
    @Categories
        => Id
    @Versions
        => Id
    @CustomFields
        => Executor
        -> CustomFieldId = "id"
    @CustomFieldId
        => Executor
    @Webhooks
        -> WebhookId = "webhook_id"
    @WebhookId
        => Executor
    @Git
        -> Repositories = "repositories"
    @Repositories
        => Executor
        => RepoIdOrName
    @RepoIdOrName
        => Executor
        -> PullRequests = "pullRequests"
    @PullRequests
        => Number
    @Number
        => Executor
        => Comments
    @Comments
        => CommentId
    @CommentId
        => Executor
);

impl_macro!(
    @Projects
        |
        |=> project_id_or_key -> ProjectIdOrKey = project_id_or_key
    @ProjectIdOrKey
        |=> issue_types -> IssueTypes
        |=> categories -> Categories
        |=> versions -> Versions
        |=> custom_fields -> CustomFields
        |=> webhooks -> Webhooks
        |=> git -> Git
        |
        |-> execute
    @IssueTypes
        |=> id -> Id
        |
    @Categories
        |=> id -> Id
        |
    @Versions
        |=> id -> Id
        |
    @CustomFields
        |
        |=> custom_field_id -> CustomFieldId = id
        |-> execute
    @CustomFieldId
        |
        |-> execute
    @Webhooks
        |
        |=> webhook_id -> WebhookId = webhook_id
    @WebhookId
        |
        |-> execute
    @Git
        |=> repositories -> Repositories
        |
    @Repositories
        |
        |=> repo_id_or_name -> RepoIdOrName = repo_id_or_name
        |-> execute
    @RepoIdOrName
        |=> pull_requests -> PullRequests
        |
        |-> execute
    @PullRequests
        |
        |=> number -> Number = number
    @Number
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
