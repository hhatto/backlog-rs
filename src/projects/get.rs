imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Projects
    ProjectIdOrKey
    Users
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
    @Users
        => Executor
);

impl_macro!(
    @Projects
        |
        |=> project_id_or_key -> ProjectIdOrKey = project_id_or_key
        |-> execute
    @ProjectIdOrKey
        |=> users -> Users
        |
        |-> execute
    @Users
        |
        |-> execute
);
