imports!();
use client::{PatchQueryBuilder, Executor};

new_type!(
    Watchings
    WatchingId
);

from!(
    @PatchQueryBuilder
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
