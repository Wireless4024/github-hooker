use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

use anyhow::{bail, Result};
use hyper::{Body, Client, Method, Request, Response};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use wildmatch::WildMatch;

const TIMEOUT: u64 = 30;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
	branch: String,
	url: String,
	secret: String,
	repo: String,
	headers: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct CHook {
	pub branch_filter: WildMatch,
	pub url: String,
	pub secret: String,
	pub repo: String,
	pub headers: Option<HashMap<String, String>>,
}

impl Hook {
	pub fn compile(self) -> CHook {
		CHook {
			branch_filter: WildMatch::new(self.branch.as_str()),
			url: self.url,
			repo: self.repo,
			secret: self.secret,
			headers: self.headers,
		}
	}

	pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
		let reader = BufReader::new(File::open(path)?);
		return Ok(serde_json::from_reader(reader)?);
	}
}

impl CHook {
	pub fn check(&self, branch: &str, repo: &str) -> bool {
		self.branch_filter.matches(branch) && self.repo.as_str() == repo
	}

	pub async fn trigger(&self) -> Result<Response<Body>> {
		let client = Client::new();
		let mut builder = Request::builder()
			.method(Method::GET)
			.uri(self.url.as_str())
			.header("User-Agent", "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36");
		if let Some(headers) = self.headers.as_ref() {
			for x in headers.iter() {
				builder = builder.header(x.0, x.1);
			}
		}
		let fut = client.request(builder.body(Body::empty())?);
		let resp = timeout(Duration::from_secs(TIMEOUT), fut).await??;
		Ok(resp)
	}

	pub async fn exec<F>(&self, branch: &str, repo: &str, verify_secret: F) -> Result<Response<Body>> where F:FnOnce(&str)->bool{
		if self.check(branch, repo) && verify_secret(self.secret.as_str()) {
			let resp = self.trigger().await?;
			Ok(resp)
		} else {
			bail!("")
		}
	}
}