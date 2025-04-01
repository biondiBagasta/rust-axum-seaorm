use serde::{ Deserialize, Serialize };
use crate::model::category_model::CategoryData;
use crate::model::pagination_model::PaginationResponse;

#[derive(Serialize)]
pub struct ProductWithCategoryData {
	pub id: i32,
	pub name: String,
	pub description: String,
	pub purchase_price: i32,
	pub selling_price: i32,
	pub stock: i32,
	pub discount: i32,
	pub image: String,
	pub category_id: i32,
	pub category: CategoryData,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime
}

#[derive(Serialize)]
pub struct ProductData {
	pub id: i32,
	pub name: String,
	pub description: String,
	pub purchase_price: i32,
	pub selling_price: i32,
	pub stock: i32,
	pub discount: i32,
	pub image: String,
	pub category_id: i32,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime
}

#[derive(Serialize)]
pub struct ProductPaginate {
	pub data: Vec<ProductWithCategoryData>,
	pub paginate: PaginationResponse
}

#[derive(Deserialize)]
pub struct ProductCreateDto {
	pub name: String,
	pub description: String,
	pub purchase_price: i32,
	pub selling_price: i32,
	pub stock: i32,
	pub discount: i32,
	pub image: String,
	pub category_id: i32
}

#[derive(Deserialize)]
pub struct ProductUpdateDto {
	pub name: Option<String>,
	pub description: Option<String>,
	pub purchase_price: Option<i32>,
	pub selling_price: Option<i32>,
	pub stock: Option<i32>,
	pub discount: Option<i32>,
	pub image: Option<String>,
	pub category_id: Option<i32>
}