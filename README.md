# backlog-rs

[Nulab's Backlog](https://developer.nulab-inc.com/docs/backlog/) API for Rust.
this implementation is based on [github-rs](https://github.com/mgattozzi/github-rs),
written by [Michael Gattozzi](https://github.com/mgattozzi).

**This is experimental module. Highly under development.**


## Usage

```toml
# Cargo.toml
backlog = { git = "https://github.com/hhatto/backlog-rs.git", branch = "master" }
```

```rust
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
                for project in json.as_array().unwrap() {
                    println!("{}", project);
                }
            }
        }
        Err(e) => println!("error={}", e),
    }
}
```

