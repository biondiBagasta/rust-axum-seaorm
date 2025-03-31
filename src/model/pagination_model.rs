use serde::{ Deserialize, Serialize };

#[derive(Serialize)]
pub struct PaginateResponse {
	pub per_page: i64,
	pub total_page: i64,
	pub count: i64,
	pub current_page: i64
}

#[derive(Deserialize)]
pub struct PaginationBody {
	pub term: String,
	pub page: i64
}