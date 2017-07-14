imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Watchings
    WatchingId
);

from!(
    @GetQueryBuilder
        -> Watchings = "watchings"
    @Watchings
        => WatchingId
    @WatchingId
        => Executor
);

impl_macro!(
    @Watchings
        |
        |=> watching_id -> WatchingId = watching_id
    @WatchingId
        |
        |-> execute
);
