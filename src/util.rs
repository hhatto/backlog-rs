use hyper::Uri;
use hyper::error::UriError;
use std::str::FromStr;
use url::form_urlencoded;

/// Add an extra subdirectory to the end of the url. This utilizes
/// Hyper's more generic Uri type. We've set it up to act as a Url.
pub fn url_join(url: &Uri, path: &str) -> Result<Uri, UriError> {
    // Absolutely hackish but don't know anything better
    match (url.scheme(), url.authority(), url.path(), url.query()) {
        (Some(s), Some(a), p, q) => {
            let mut curr_path = String::from(s);
            curr_path += "://";
            curr_path += a;
            curr_path += p;
            if !curr_path.ends_with('/') {
                curr_path.push('/');
            }
            curr_path.push_str(path);
            if let Some(query) = q {
                curr_path.push('?');
                curr_path += query;
            }
            curr_path.parse::<Uri>()
        }
        // This should cause the request to fail if something goes
        // wrong.
        _ => Uri::from_str("Failed to make a valid Url"),
    }
}

pub fn url_add_query(url: &Uri, params: Vec<(&str, &str)>) -> Result<Uri, UriError> {
    // Absolutely hackish but don't know anything better
    match (url.scheme(), url.authority(), url.path(), url.query()) {
        (Some(s), Some(a), p, q) => {
            let mut curr_path = String::from(s);
            curr_path += "://";
            curr_path += a;
            curr_path += p;
            if !curr_path.ends_with('/') {
                curr_path.push('/');
            }
            match q {
                Some(query) => {
                    curr_path.push('?');
                    curr_path += query;
                    let mut s = form_urlencoded::Serializer::new(String::new());
                    for (key, value) in params {
                        s.append_pair(key, value);
                    }
                    curr_path += format!("&{}", s.finish()).as_str();
                }
                None => {
                    if !params.is_empty() {
                        curr_path.push('?');
                        for param in params {
                            let (key, value) = param;
                            curr_path += format!("&{}={}", key, value).as_str();
                        }
                    }
                }
            }
            curr_path.parse::<Uri>()
        }
        // This should cause the request to fail if something goes
        // wrong.
        _ => Uri::from_str("Failed to make a valid Url"),
    }
}
