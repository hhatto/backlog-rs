imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Notifications
    NotificationId
    MarkAsRead
);

from!(
    @PostQueryBuilder
        -> Notifications = "notifications"
    @Notifications
        -> NotificationId = "id"
        -> MarkAsRead = "markAsRead"
    @NotificationId
        -> MarkAsRead = "markAsRead"
    @MarkAsRead
        => Executor
);

impl_macro!(
    @Notifications
        |=> mark_as_read -> MarkAsRead
        |
        |=> notification_id -> NotificationId = notification_id
    @NotificationId
        |=> mark_as_read -> MarkAsRead
        |
    @MarkAsRead
        |
        |-> execute
);
