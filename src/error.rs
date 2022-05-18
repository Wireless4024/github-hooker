use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use anyhow::Error;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InternalError {
	message: String,
}

impl Display for InternalError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.message.as_str())
	}
}

impl StdError for InternalError {}

impl From<Error> for InternalError{
	fn from(err: Error) -> Self {
		// TODO: implement backtrace
		Self{message:err.to_string()}
	}
}

impl IntoResponse for InternalError {
	fn into_response(self) -> Response {
		(StatusCode::INTERNAL_SERVER_ERROR, serde_json::to_string(&self).unwrap()).into_response()
	}
}