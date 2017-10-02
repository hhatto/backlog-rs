imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Watchings
    WatchId
    MarkAsRead
);

from!(
    @PostQueryBuilder
        -> Watchings = "watchings"
    @Watchings
        => Executor
        -> WatchId = "watchId"
    @WatchId
        -> MarkAsRead = "markAsRead"
    @MarkAsRead
        => Executor
);

impl_macro!(
    @Watchings
        |
        |=> watch_id -> WatchId = watch_id
        |-> execute
    @WatchId
        |=> mark_as_read -> MarkAsRead
        |
    @MarkAsRead
        |
        |-> execute
);
