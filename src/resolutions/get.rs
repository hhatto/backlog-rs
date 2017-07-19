imports!();
use client::{GetQueryBuilder, Executor};

new_type!(Resolutions);

from!(
    @GetQueryBuilder
        -> Resolutions = "resolutions"
    @Resolutions
        => Executor
);

impl_macro!(
    @Resolutions
        |
        |-> execute
);
