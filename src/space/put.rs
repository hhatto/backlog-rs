imports!();
use client::{PutQueryBuilder, Executor};

new_type!(
    Space
    Notification
);

from!(
    @PutQueryBuilder
        -> Space = "space"
    @Space
        -> Notification = "notification"
    @Notification
        => Executor
);

impl_macro!(
    @Space
        |=> notification -> Notification
        |
    @Notification
        |
        |-> execute
);
