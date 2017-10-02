imports!();
use client::{DeleteQueryBuilder, Executor};

new_type!(
    Projects
    ProjectIdOrKey
    Users
    Administrators
    IssueTypes
    Categories
    Versions
    CustomFields
    CustomFieldId
    Items
    ItemId
    Id
    Webhooks
    WebhookId
    Git
    Repositories
    RepoIdOrName
    PullRequests
    Number
    Attachments
    AttachmentId
);

from!(
    @DeleteQueryBuilder
        -> Projects = "projects"
    @Projects
        => ProjectIdOrKey
    @ProjectIdOrKey
        => Executor
        -> Users = "users"
        -> Administrators = "administrators"
        -> IssueTypes = "issueTypes"
        -> Categories = "categories"
        -> Versions = "versions"
        -> CustomFields = "customFields"
        -> Webhooks = "webhooks"
        -> Git = "git"
    @Users
        => Executor
    @Administrators
        => Executor
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
        -> Items = "items"
    @Items
        -> ItemId = "item_id"
    @ItemId
        => Executor
    @Webhooks
        -> WebhookId = "webhook_id"
    @WebhookId
        => Executor
    @Git
        -> Repositories = "repositories"
    @Repositories
        => RepoIdOrName
    @RepoIdOrName
        -> PullRequests = "pullRequests"
    @PullRequests
        => Number
    @Number
        => Attachments
    @Attachments
        -> AttachmentId = "attachment_id"
    @AttachmentId
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
    @RepoIdOrName
        |=> pull_requests -> PullRequests
        |
    @PullRequests
        |
        |=> number -> Number = number
    @Number
        |=> attachments -> Attachments
        |
    @Attachments
        |
        |=> attachment_id -> AttachmentId = attachment_id
    @AttachmentId
        |
        |-> execute
);
