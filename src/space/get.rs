imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Space
    Activities
    Image
    Notification
    DiskUsage
);

from!(
    @GetQueryBuilder
        -> Space = "space"
    @Space
        => Executor
        -> Activities = "activities"
        -> Image = "image"
        -> Notification = "notification"
        -> DiskUsage = "diskUsage"
    @Activities
        => Executor
    @Image
        => Executor
    @Notification
        => Executor
    @DiskUsage
        => Executor
);

impl_macro!(
    @Space
        |=> activities -> Activities
        |=> image -> Image
        |=> notification -> Notification
        |=> disk_usage -> DiskUsage
        |
        |-> execute
    @Activities
        |
        |-> execute
    @Image
        |
        |-> execute
    @Notification
        |
        |-> execute
    @DiskUsage
        |
        |-> execute
);
