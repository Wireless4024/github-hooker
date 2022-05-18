use serde::Deserialize;

use super::common::{GithubPublicUser, GithubRepository, GithubUser};

#[derive(Deserialize, Debug)]
pub struct GithubWebhookPushPayload {
	#[serde(rename = "ref")]
	pub git_ref: String,
	pub base_ref: Option<String>,
	pub before: String,
	pub after: String,
	pub created: bool,
	pub deleted: bool,
	pub forced: bool,
	// head_commit
	pub compare: String,
	pub commits: Vec<GithubWebhookPushCommit>,
	pub pusher: GithubPublicUser,
	pub sender: GithubUser,
	pub repository: GithubRepository,
}

#[derive(Deserialize, Debug)]
pub struct GithubWebhookPushCommit {
	pub id: String,
	pub timestamp: String,
	pub message: String,
	pub author: GithubPublicUser,
	pub url: String,
	pub distinct: bool,
	pub added: Vec<String>,
}

impl GithubWebhookPushPayload {
	pub fn branch(&self) -> &str {
		let mut iter = self.git_ref.as_str().split('/');
		let mut latest = iter.next().unwrap();
		while let Some(cur) = iter.next() {
			latest = cur
		}
		latest
	}
}