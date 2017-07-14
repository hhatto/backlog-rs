use futures::{Future, Stream};
use futures::future::ok;
use tokio_core::reactor::Core;
use hyper::{Body, Headers, Method, StatusCode, Uri, Error};
use hyper::client::{Client, HttpConnector, Request};
use hyper::header::{Accept, ContentType, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;
use serde_json;

use groups;
use projects;
use space;
use users;
use watchings;
use errors::*;
use util::url_join;

use Json;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Backlog {
    group: String,
    api_key: String,
    core: Rc<RefCell<Core>>,
    client: Rc<Client<HttpsConnector<HttpConnector>>>,
}

new_type!(GetQueryBuilder);
new_type!(CustomQuery);
new_type!(Executor);

impl Backlog {
    pub fn new<T>(group: &str, api_key: T) -> Result<Self>
        where T: AsRef<str>
    {
        let core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).expect("HttpsConnector fail"))
            .build(&handle);
        Ok(Self {
            group: group.to_string(),
            api_key: api_key.as_ref().into(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        })
    }

    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }
}

impl<'g> GetQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    func_client!(projects, projects::get::Projects<'g>);
    func_client!(space, space::get::Space<'g>);
    func_client!(users, users::get::Users<'g>);
    func_client!(watchings, watchings::get::Watchings<'g>);
    func_client!(groups, groups::get::Groups<'g>);
}

impl<'g> Executor<'g> {
    pub fn execute(self) -> Result<(Headers, StatusCode, Option<Json>)> {
        let mut core_ref = self.core
            .try_borrow_mut()
            .chain_err(|| "Unable to get mutable borrowto the event loop")?;
        let client = self.client;
        let work = client.request(self.request?.into_inner())
            .and_then(|res| {
                let header = res.headers().clone();
                let status = res.status();
                res.body()
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        ok::<_, Error>(v)
                    })
                    .map(move |chunks| {
                        if chunks.is_empty() {
                            (header, status, None)
                        } else {
                            (header, status, Some(serde_json::from_slice(&chunks).unwrap()))
                        }
                    })
            });
        core_ref.run(work).chain_err(|| "Failed to execute request")
    }
}

from!(
    @GetQueryBuilder
        => Method::Get
);

from!(
    @GetQueryBuilder
       => CustomQuery
    @CustomQuery
        => Executor
);
