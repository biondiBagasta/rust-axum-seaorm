use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub full_name: String,
	pub address: String,
	pub phone_number: String,
	pub role: String,
	pub photo: String,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime
}

#[derive(Deserialize)]
pub struct UserCreateBody {
	pub username: String,
	pub password: String,
	pub full_name: String,
	pub address: String,
	pub phone_number: String,
	pub role: String,
	pub photo: String
}

#[derive(Deserialize)]
pub struct UserUpdateBody {
	pub username: Option<String>,
	pub password: Option<String>,
	pub full_name: Option<String>,
	pub address: Option<String>,
	pub phone_number: Option<String>,
	pub role: Option<String>,
	pub photo: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
	pub user_data: UserData,
	pub exp: usize,
}
