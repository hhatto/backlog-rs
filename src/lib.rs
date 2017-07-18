#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
extern crate futures;

pub type Json = serde_json::Value;

#[macro_use]
mod macros;
pub mod client;
mod util;

pub mod errors {
    error_chain!{}
}

pub mod projects;
pub mod space;
pub mod users;
pub mod groups;
pub mod notifications;
pub mod watchings;
pub mod wikis;
pub mod priorities;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
