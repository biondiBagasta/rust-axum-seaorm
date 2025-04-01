use serde::{ Deserialize, Serialize };
use crate::model::pagination_model::{ PaginationResponse };

#[derive(Serialize)]
pub struct CategoryData {
	pub id: i32,
	pub name: String,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime
}

#[derive(Serialize)]
pub struct CategoryPaginate {
	pub data: Vec<CategoryData>,
	pub paginate: PaginationResponse
}

#[derive(Deserialize)]
pub struct CategoryCreateBody {
	pub name: String
}

#[derive(Deserialize)]
pub struct CategoryUpdateBody {
	pub name: Option<String>
}