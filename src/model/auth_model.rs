use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginBody {
	pub username: String,
	pub password: String
}

#[derive(Deserialize)]
pub struct ChangePasswordBody {
	pub id: i32,
	pub old_password: String,
	pub new_password: String
}