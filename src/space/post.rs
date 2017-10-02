imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Space
    Attachment
);

from!(
    @PostQueryBuilder
        -> Space = "space"
    @Space
        -> Attachment = "attachment"
    @Attachment
        => Executor
);

impl_macro!(
    @Space
        |=> attachment -> Attachment
        |
    @Attachment
        |
        |-> execute
);
