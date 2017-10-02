imports!();
use client::{PostQueryBuilder, Executor};

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
    Id
    Webhooks
    Git
    Repositories
    RepoIdOrName
    PullRequests
    Number
    Comments
);

from!(
    @PostQueryBuilder
        -> Projects = "projects"
    @Projects
        => Executor
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
        -> Items = "items"
    @Items
        => Executor
    @Webhooks
        => Executor
    @Git
        -> Repositories = "repositories"
    @Repositories
        => RepoIdOrName
    @RepoIdOrName
        -> PullRequests = "pullRequests"
    @PullRequests
        => Executor
        => Number
    @Number
        => Comments
    @Comments
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
        |=> items -> Items
        |
    @Items
        |
        |-> execute
    @Webhooks
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
        |=> comments -> Comments
        |
    @Comments
        |
        |-> execute
);
