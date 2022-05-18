use std::net::SocketAddr;

use axum::Router;
use dotenv::var;
use tower_http::cors::CorsLayer;

use crate::hooker::hook::Hook;
use crate::route::{load_hooks, mk_route_v1};

mod route;
mod github;
mod error;
mod hooker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv::dotenv().ok();
	load_hooks().await?;
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "info")
	}
	tracing_subscriber::fmt::init();
	let app = Router::new()
		.nest("/api/v1", mk_route_v1())
		.layer(CorsLayer::permissive());

	let port = var("PORT").ok().and_then(|it| it.parse::<u16>().ok());

	let addr = SocketAddr::from(([0, 0, 0, 0], port.unwrap_or(8887)));
	axum::Server::bind(&addr)
		.tcp_nodelay(true)
		.http1_half_close(true)
		.http1_max_buf_size(65536)
		.serve(app.into_make_service())
		.await?;
	Ok(())
}
