extern crate backlog;
extern crate hyper;

use backlog::client::Backlog;
use hyper::StatusCode;

static BACKLOG_GROUP: &'static str = "GROUP";
static API_KEY: &'static str = "API KEY";

fn main() {
    let client = Backlog::new(BACKLOG_GROUP, API_KEY).expect("new backlog fail");
    let projects = client.get().projects().execute();
    match projects {
        Ok((headers, status, json)) => {
            println!("{}, {}", headers, status);
            match status {
                StatusCode::Ok => {}
                _ => {
                    println!("status={}, res={}", status, json);
                    return
                }
            }
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("error={}", e),
    }
}
