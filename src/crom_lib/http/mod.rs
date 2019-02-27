use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;

use reqwest::header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT, HeaderMap};
use reqwest::Request;

use mime::Mime;
use mime_guess::guess_mime_type;
use url::Url;

use crate::crom_lib::error::*;

pub fn make_file_upload_request(
    url: &Url,
    file_path: PathBuf,
    headers: HashMap<HeaderName, HeaderValue>,
) -> Result<Request, ErrorContainer> {
    debug!("Upload url {}", url);

    let mime: Mime = guess_mime_type(&file_path);

    if !file_path.exists() {
        return Err(ErrorContainer::IO(IOError::FileNotFound(file_path.clone())));
    }
    let mut buffer = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        header_map.insert(key, value);
    }

    Ok(crate::crom_lib::client()
        .post(url.as_str())
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(CONTENT_TYPE, mime.to_string())
        .headers(header_map)
        .body(buffer)
        .build()
        .unwrap())
}

pub fn make_get_request(
    url: &str,
    headers: HashMap<HeaderName, HeaderValue>,
) -> Result<Request, ErrorContainer> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        header_map.insert(key, value);
    }

    Ok(crate::crom_lib::client()
        .get(url)
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .headers(header_map)
        .build()
        .unwrap())
}

pub fn make_post(
    url: &str,
    headers: HashMap<HeaderName, HeaderValue>,
    body_content: String,
) -> Result<Request, ErrorContainer> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        header_map.insert(key, value);
    }

    Ok(crate::crom_lib::client()
        .post(url)
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .headers(header_map)
        .body(body_content)
        .build()
        .unwrap())
}

#[cfg(test)]
pub fn make_github_auth_headers() -> Result<HashMap<HeaderName, HeaderValue>, ErrorContainer> {
    warn!("Using debug GITHUB headers!");
    Ok(HashMap::new())
}

#[cfg(not(test))]
pub fn make_github_auth_headers() -> Result<HashMap<HeaderName, HeaderValue>, ErrorContainer> {
    use reqwest::header::AUTHORIZATION;

    let token = match std::env::var("GITHUB_TOKEN") {
        Ok(value) => format!("bearer {}", value),
        Err(_) => return Err(ErrorContainer::GitHub(GitHubError::TokenMissing)),
    };

    let value = match HeaderValue::from_str(&token) {
        Ok(it) => it,
        Err(err) => {
            return Err(ErrorContainer::GitHub(GitHubError::TokenInvalid(
                err.to_string(),
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
