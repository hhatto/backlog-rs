imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Projects
    ProjectIdOrKey
    Users
    Image
    Activities
    Administrators
    IssueTypes
    Categories
    Versions
    CustomFields
    Files
    Metadata
    SharedFileId
    Path
    DiskUsage
    Webhooks
    WebhookId
    Git
    Repositories
    RepoIdOrName
    PullRequests
    Count
    Number
    Comments
    CommentsCount
    Attachments
    AttachmentId
);

from!(
    @GetQueryBuilder
        -> Projects = "projects"
    @Projects
        => Executor
        => ProjectIdOrKey
    @ProjectIdOrKey
        => Executor
        -> Users = "users"
        -> Image = "image"
        -> Activities = "activities"
        -> Administrators = "administrators"
        -> IssueTypes = "issueTypes"
        -> Categories = "categories"
        -> Versions = "versions"
        -> CustomFields = "customFields"
        -> Files = "files"
        -> DiskUsage = "diskUsage"
        -> Webhooks = "webhooks"
        -> Git = "git"
    @Users
        => Executor
    @Image
        => Executor
    @Activities
        => Executor
    @Administrators
        => Executor
    @IssueTypes
        => Executor
    @Categories
        => Executor
    @Versions
        => Executor
    @CustomFields
        => Executor
    @Files
        => SharedFileId
        -> Metadata = "metadata"
    @Metadata
        -> Path = "path"
    @SharedFileId
        => Executor
    @Path
        => Executor
    @DiskUsage
        => Executor
    @Webhooks
        => Executor
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
        => Executor
        => Count
        => Number
    @Count
        => Executor
    @Number
        => Executor
        => Comments
        => Attachments
    @Comments
        => Executor
        => CommentsCount
    @CommentsCount
        => Executor
    @Attachments
        => Executor
        => AttachmentId
    @AttachmentId
        => Executor
);

impl_macro!(
    @Projects
        |
        |=> project_id_or_key -> ProjectIdOrKey = project_id_or_key
        |-> execute
    @ProjectIdOrKey
        |=> image -> Image
        |=> users -> Users
        |=> activities -> Activities
        |=> administrators -> Administrators
        |=> issue_types -> IssueTypes
        |=> categories -> Categories
        |=> versions -> Versions
        |=> custom_fields -> CustomFields
        |=> files -> Files
        |=> disk_usage -> DiskUsage
        |=> webhooks -> Webhooks
        |=> git -> Git
        |
        |-> execute
    @Users
        |
        |-> execute
    @Image
        |
        |-> execute
    @Activities
        |
        |-> execute
    @Administrators
        |
        |-> execute
    @IssueTypes
        |
        |-> execute
    @Categories
        |
        |-> execute
    @Versions
        |
        |-> execute
    @CustomFields
        |
        |-> execute
    @Files
        |=> metadata -> Metadata
        |
        |=> shared_file_id -> SharedFileId = shared_file_id
    @Metadata
        |
        |=> path -> Path = path
    @SharedFileId
        |
        |-> execute
    @Path
        |
        |-> execute
    @DiskUsage
        |
        |-> execute
    @Webhooks
        |
        |=> webhook_id -> WebhookId = webhook_id
        |-> execute
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
        |=> count -> Count
        |
        |=> number -> Number = number
        |-> execute
    @Count
        |
        |-> execute
    @Number
        |=> comments -> Comments
        |=> attachments -> Attachments
        |
        |-> execute
    @Comments
        |=> count -> CommentsCount
        |
        |-> execute
    @CommentsCount
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
