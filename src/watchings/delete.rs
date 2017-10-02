imports!();
use client::{DeleteQueryBuilder, Executor};

new_type!(
    Watchings
    WatchingId
);

from!(
    @DeleteQueryBuilder
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
