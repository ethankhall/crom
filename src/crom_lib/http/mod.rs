use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use hyper::body::Body;
use hyper::header::{HeaderName, HeaderValue, ACCEPT, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use hyper::Request;

use mime::Mime;
use mime_guess::guess_mime_type;
use std::io::prelude::*;
use url::Url;

use crate::crom_lib::error::*;

pub fn make_file_upload_request(
    url: &Url,
    file_path: PathBuf,
    headers: HashMap<HeaderName, HeaderValue>,
) -> Result<Request<Body>, ErrorContainer> {
    debug!("Upload url {}", url);

    let mime: Mime = guess_mime_type(&file_path);

    let mut file = File::open(file_path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;

    let size = contents.len();

    let mut builder = Request::builder();
    let builder = builder.method("POST").uri(url.as_str());

    for (key, value) in headers {
        builder.header(key, value);
    }

    Ok(Request::builder()
        .method("POST")
        .uri(url.as_str())
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(CONTENT_TYPE, mime.to_string())
        .header(CONTENT_LENGTH, size)
        .body(contents.into())
        .unwrap())
}

pub fn make_get_request(
    url: &str,
    headers: HashMap<HeaderName, HeaderValue>,
) -> Result<Request<Body>, ErrorContainer> {
    let mut builder = Request::builder();
    let builder = builder.method("GET").uri(url);

    for (key, value) in headers {
        builder.header(key, value);
    }

    Ok(builder
        .uri(url)
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .body(Body::empty())
        .unwrap())
}

pub fn make_post(
    url: &str,
    headers: HashMap<HeaderName, HeaderValue>,
    body_content: String,
) -> Result<Request<Body>, ErrorContainer> {
    let mut builder = Request::builder();
    let builder = builder.method("POST").uri(url);

    for (key, value) in headers {
        builder.header(key, value);
    }

    Ok(builder
        .uri(url)
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .body(body_content.into())
        .unwrap())
}

#[cfg(test)]
pub fn make_github_auth_headers() -> Result<HashMap<HeaderName, HeaderValue>, ErrorContainer> {
    Ok(HashMap::new())
}

#[cfg(not(test))]
pub fn make_github_auth_headers() -> Result<HashMap<HeaderName, HeaderValue>, ErrorContainer> {
    use hyper::header::AUTHORIZATION;
    use std::error::Error;

    let token = match std::env::var("GITHUB_TOKEN") {
        Ok(value) => format!("token {}", value),
        Err(_) => return Err(ErrorContainer::GitHub(GitHubError::TokenMissing)),
    };

    let value = match HeaderValue::from_str(&token) {
        Ok(it) => it,
        Err(err) => {
            return Err(ErrorContainer::GitHub(GitHubError::TokenInvalid(
                err.description().to_string(),
            )));
        }
    };

    let mut map: HashMap<HeaderName, HeaderValue> = HashMap::new();
    map.insert(AUTHORIZATION, value);

    Ok(map)
}

#[cfg(test)]
use mockito;

#[cfg(not(test))]
pub fn get_github_api() -> String {
    use std::env;

    match env::var("GITHUB_API_SERVER") {
        Ok(value) => value,
        Err(_) => s!("https://api.github.com"),
    }
}

#[cfg(test)]
pub fn get_github_api() -> String {
    mockito::server_url()
}
