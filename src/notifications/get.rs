imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Notifications
    Count
);

from!(
    @GetQueryBuilder
        -> Notifications = "notifications"
    @Notifications
        => Executor
        -> Count = "count"
    @Count
        => Executor
);

impl_macro!(
    @Notifications
        |=> count -> Count
        |
        |-> execute
    @Count
        |
        |-> execute
);
