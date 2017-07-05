extern crate backlog;

use backlog::client::Backlog;

static BACKLOG_GROUP: &'static str = "GROUP";
static API_KEY: &'static str = "API KEY";

fn main() {
    let client = Backlog::new(BACKLOG_GROUP, API_KEY).expect("new backlog fail");
    let projects = client.get().projects().execute();
    match projects {
        Ok((headers, status, json)) => {
            println!("{}, {}", headers, status);
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("error={}", e),
    }
}
