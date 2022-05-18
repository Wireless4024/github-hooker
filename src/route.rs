use std::env::var;
use std::ops::Deref;

use anyhow::Result;
use axum::{Json, Router};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use sha1::Sha1;
use tokio::fs::read_dir;
use tokio::sync::RwLock;

use crate::error::InternalError;
use crate::github::push_hook::{GithubWebhookPushPayload};
use crate::Hook;
use crate::hooker::hook::CHook;

pub fn mk_route_v1() -> Router {
	Router::new()
		.route("/push", post(gh_push))
		.route("/reload_me_please", get(reload))
}

async fn reload() -> impl IntoResponse {
	dotenv::dotenv().ok();
	load_hooks().await.ok();
	""
}

static HOOKS: RwLock<Vec<CHook>> = RwLock::const_new(Vec::new());

pub async fn load_hooks() -> Result<()> {
	let scan_dir = var("HOOK_FOLDER").unwrap_or_else(|_| String::from("hooks"));
	let mut dir = read_dir(scan_dir).await?;
	let mut hooks = Vec::new();
	while let Ok(Some(e)) = dir.next_entry().await {
		let path = e.path();
		if let Some(false) = path.extension().map(|it| it == "json") {
			continue;
		}
		if let Ok(hook) = Hook::new(e.path()).await {
			hooks.push(hook.compile());
		}
	}
	let mut hooks_const = HOOKS.write().await;
	*hooks_const = hooks;
	Ok(())
}

pub trait VerifyGhSecret {
	fn verify_gh_secret(&self, secret: &str) -> bool;
}

impl VerifyGhSecret for HeaderMap {
	fn verify_gh_secret(&self, secret: &str) -> bool {
		use sha1::Digest;
		let mut dg = Sha1::default();
		dg.update(secret);
		let secret = format!("{:X?}", dg.finalize().as_slice());
		self.get("x-hub-signature").map(|it| &(it.as_bytes()[5..]) == secret.as_bytes()).unwrap_or(false)
	}
}

async fn gh_push(Json(body): Json<GithubWebhookPushPayload>) -> Result<String, InternalError> {
	tokio::spawn(async move {
		let hooks = HOOKS.read().await;
		let a = hooks.deref();
		for x in a {
			x.exec(body.branch(), body.repository.name.as_str(), |_| true).await.ok();
		}
	});
	Ok("Ok".to_string())
}