use error_chain::bail;
use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(test)]
use log::warn;

use crate::errors::ErrorKind;
use crate::CromResult;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::Request;

use mime_guess::from_path;
use url::Url;

pub fn make_file_upload_request(
    url: &Url,
    file_path: PathBuf,
    headers: HashMap<String, String>,
) -> CromResult<Request> {
    debug!("Upload url {}", url);

    let mime = from_path(&file_path).first_or_octet_stream();

    if !file_path.exists() {
        bail!(ErrorKind::FileNotFound(file_path));
    }
    let mut buffer = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;

    let header_map = build_headers(headers)?;

    Ok(client()
        .post(url.as_str())
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(CONTENT_TYPE, mime.to_string())
        .headers(header_map)
        .body(buffer)
        .build()
        .unwrap())
}

pub fn make_get_request(url: &str, headers: HashMap<String, String>) -> CromResult<Request> {
    let header_map = build_headers(headers)?;

    Ok(client()
        .get(url)
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .headers(header_map)
        .build()
        .unwrap())
}

pub fn make_post(
    url: &str,
    headers: HashMap<String, String>,
    body_content: String,
) -> CromResult<Request> {
    let header_map = build_headers(headers)?;

    Ok(client()
        .post(url)
        .header(USER_AGENT, format!("crom/{}", env!("CARGO_PKG_VERSION")))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .headers(header_map)
        .body(body_content)
        .build()
        .unwrap())
}

fn build_headers(headers: HashMap<String, String>) -> CromResult<HeaderMap> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        let value = match HeaderValue::from_str(&value) {
            Ok(value) => value,
            Err(e) => bail!(ErrorKind::HeaderError(e.to_string())),
        };
        let key = match HeaderName::from_str(&key) {
            Ok(key) => key,
            Err(e) => bail!(ErrorKind::HeaderError(e.to_string())),
        };

        header_map.insert(key, value);
    }

    Ok(header_map)
}

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .gzip(true)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

#[cfg(test)]
pub fn make_github_auth_headers(_auth: &str) -> CromResult<HashMap<String, String>> {
    warn!("Using debug GITHUB headers!");
    Ok(HashMap::new())
}

#[cfg(not(test))]
pub fn make_github_auth_headers(auth: &str) -> CromResult<HashMap<String, String>> {
    use reqwest::header::AUTHORIZATION;

    let token = format!("bearer {}", auth);

    let mut map: HashMap<String, String> = HashMap::new();
    let auth_header = AUTHORIZATION;
    map.insert(auth_header.to_string(), token);

    Ok(map)
}

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
