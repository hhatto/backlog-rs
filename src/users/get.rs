imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Users
    UserId
    Myself
    Icon
    Activities
    Stars
    Count
    RecentlyViewedIssues
    RecentlyViewedProjects
    RecentlyViewedWikis
    Watchings
);

from!(
    @GetQueryBuilder
        -> Users = "users"
    @Users
        => Executor
        => UserId
        -> Myself = "myself"
    @UserId
        => Executor
        -> Icon = "icon"
        -> Activities = "activities"
        -> Stars = "stars"
        -> Watchings = "watchings"
    @Myself
        => Executor
        -> RecentlyViewedIssues = "recentlyViewedIssues"
        -> RecentlyViewedProjects = "recentlyViewedProjects"
        -> RecentlyViewedWikis = "recentlyViewedWikis"
    @Icon
        => Executor
    @Activities
        => Executor
    @Stars
        => Executor
        -> Count = "count"
    @Count
        => Executor
    @RecentlyViewedIssues
        => Executor
    @RecentlyViewedProjects
        => Executor
    @RecentlyViewedWikis
        => Executor
    @Watchings
        => Executor
        -> Count = "count"
);

impl_macro!(
    @Users
        |=> myself -> Myself
        |
        |=> user_id -> UserId = user_id
        |-> execute
    @UserId
        |=> icon -> Icon
        |=> activities -> Activities
        |=> stars -> Stars
        |=> watchings -> Watchings
        |
        |-> execute
    @Myself
        |=> recently_viewed_issues -> RecentlyViewedIssues
        |=> recently_viewed_projects -> RecentlyViewedProjects
        |=> recently_viewed_wikis -> RecentlyViewedWikis
        |
        |-> execute
    @Icon
        |
        |-> execute
    @Activities
        |
        |-> execute
    @Stars
        |=> count -> Count
        |
        |-> execute
    @Count
        |
        |-> execute
    @RecentlyViewedIssues
        |
        |-> execute
    @RecentlyViewedProjects
        |
        |-> execute
    @RecentlyViewedWikis
        |
        |-> execute
    @Watchings
        |=> count -> Count
        |
        |-> execute
);
