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
use notifications;
use wikis;
use priorities;
use resolutions;
use statuses;
use issues;
use errors::*;
use util::{url_join, url_add_query};

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
new_type!(PostQueryBuilder);
new_type!(PatchQueryBuilder);
new_type!(PutQueryBuilder);
new_type!(DeleteQueryBuilder);
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

    pub fn get_with_params(&self, body: Vec<(&str, &str)>) -> GetQueryBuilder {
        let mut gb: GetQueryBuilder = self.into();
        if let Ok(mut gbr) = gb.request {
            let new_uri = {
                let uri = gbr.get_mut().uri();
                url_add_query(uri, body).expect("build query error")
            };
            gbr.get_mut().set_uri(new_uri);
            gb.request = Ok(gbr);
        }
        gb
    }

    pub fn post(&self) -> PostQueryBuilder {
        self.into()
    }

    pub fn post_with_params(&self, body: Vec<(&str, &str)>) -> PostQueryBuilder {
        let mut gb: PostQueryBuilder = self.into();
        if let Ok(mut gbr) = gb.request {
            let new_uri = {
                let uri = gbr.get_mut().uri();
                url_add_query(uri, body).expect("build query error")
            };
            gbr.get_mut().set_uri(new_uri);
            gb.request = Ok(gbr);
        }
        gb
    }

    pub fn patch_with_params(&self, body: Vec<(&str, &str)>) -> PatchQueryBuilder {
        let mut gb: PatchQueryBuilder = self.into();
        if let Ok(mut gbr) = gb.request {
            let new_uri = {
                let uri = gbr.get_mut().uri();
                url_add_query(uri, body).expect("build query error")
            };
            gbr.get_mut().set_uri(new_uri);
            gb.request = Ok(gbr);
        }
        gb
    }

    pub fn put(&self) -> PutQueryBuilder {
        self.into()
    }

    pub fn delete(&self) -> DeleteQueryBuilder {
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
    func_client!(notifications, notifications::get::Notifications<'g>);
    func_client!(wikis, wikis::get::Wikis<'g>);
    func_client!(priorities, priorities::get::Priorities<'g>);
    func_client!(resolutions, resolutions::get::Resolutions<'g>);
    func_client!(statuses, statuses::get::Statuses<'g>);
    func_client!(issues, issues::get::Issues<'g>);
}

impl<'g> PostQueryBuilder<'g> {
    func_client!(projects, projects::post::Projects<'g>);
    func_client!(space, space::post::Space<'g>);
    func_client!(users, users::post::Users<'g>);
    func_client!(issues, issues::post::Issues<'g>);
    func_client!(groups, groups::post::Groups<'g>);
    func_client!(notifications, notifications::post::Notifications<'g>);
}

impl<'g> PutQueryBuilder<'g> {
    func_client!(projects, space::put::Space<'g>);
}

impl<'g> PatchQueryBuilder<'g> {
    func_client!(projects, projects::patch::Projects<'g>);
    func_client!(watchings, watchings::patch::Watchings<'g>);
    func_client!(wikis, wikis::patch::Wikis<'g>);
    func_client!(users, users::patch::Users<'g>);
    func_client!(groups, groups::patch::Groups<'g>);
    func_client!(issues, issues::patch::Issues<'g>);
}

impl<'g> DeleteQueryBuilder<'g> {
    func_client!(projects, projects::delete::Projects<'g>);
    func_client!(users, users::delete::Users<'g>);
    func_client!(issues, issues::delete::Issues<'g>);
    func_client!(groups, groups::delete::Groups<'g>);
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
    @PostQueryBuilder
        => Method::Post
    @PatchQueryBuilder
        => Method::Patch
    @PutQueryBuilder
        => Method::Put
    @DeleteQueryBuilder
        => Method::Delete
);

from!(
    @GetQueryBuilder
       => CustomQuery
    @PostQueryBuilder
       => CustomQuery
    @PatchQueryBuilder
       => CustomQuery
    @PutQueryBuilder
       => CustomQuery
    @DeleteQueryBuilder
       => CustomQuery
    @CustomQuery
        => Executor
);
