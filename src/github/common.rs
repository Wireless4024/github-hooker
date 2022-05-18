use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GithubPublicUser {
	pub name: String,
	pub email: String,
	pub username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GithubUser {
	pub name: Option<String>,
	pub email: Option<String>,
	pub login: String,
	pub id: u64,
	pub node_id: String,
	pub avatar_url: String,
	pub gravatar_id: String,
	pub url: String,
	pub html_url: String,
	#[serde(rename = "type")]
	pub user_type: String,
	pub site_admin: bool,
}

/*impl GithubUser {
	fn is_user(&self) -> bool {
		self.user_type.as_str() == "User"
	}

	fn is_organization(&self) -> bool {
		self.user_type.as_str() == "Organization"
	}
}
*/
#[derive(Deserialize, Debug)]
pub struct GithubRepository {
	pub id: u64,
	pub node_id: String,
	pub name: String,
	pub full_name: String,
	pub private: bool,
	pub owner: GithubUser,
	pub html_url: String,
	pub description: Option<String>,
	pub fork: bool,
	pub url: String,
	pub created_at: u64,
	pub updated_at: String,
	pub pushed_at: u64,
	pub clone_url: String,
	pub language: Option<String>,
}