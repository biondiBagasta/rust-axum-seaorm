use axum::{
	extract:: { Path, State }, http::StatusCode, response::IntoResponse, Json
};

use sea_orm::{
	ActiveModelTrait,
	ActiveValue::Set,
	Condition,
	DatabaseConnection,
	EntityTrait,
	QueryOrder,
	QueryFilter,
	ColumnTrait,
	PaginatorTrait
};

use serde_json::json;

use crate::model::product_model::{ ProductPaginate, ProductCreateDto, ProductUpdateDto,
ProductWithCategoryData };
use crate::model::category_model::CategoryData;

use crate::model::pagination_model::{ PaginationBody, PaginationResponse };

use entity::{ product, category };

pub async fn search_paginate(
	State(db): State<DatabaseConnection>,
	Json(body): Json<PaginationBody>
) -> impl IntoResponse {
	const PAGE_TAKE: i64 = 10;

	let query_count = product::Entity::find().filter(
		Condition::any().add(
			product::Column::Name.contains(&body.term)
		)
	).count(&db).await.unwrap();

	let query_search: Vec<ProductWithCategoryData> = product::Entity::find()
	.order_by_asc(product::Column::Name)
	.find_with_related(category::Entity)
	.filter(
		Condition::any().add(
			product::Column::Name.contains(&body.term)
		)
	).all(&db).await.unwrap().iter().into_iter().map(|d| ProductWithCategoryData {
		id: d.0.id,
		name: d.0.name.clone(),
		description: d.0.description.clone(),
		purchase_price: d.0.purchase_price,
		selling_price: d.0.selling_price,
		stock: d.0.stock,
		discount: d.0.discount,
		image: d.0.image.clone(),
		category_id: d.0.category_id,
		category: CategoryData { 
			id: d.1.get(0).unwrap().id, 
			name: d.1.get(0).unwrap().name.clone(), 
			created_at: d.1.get(0).unwrap().created_at, 
			updated_at: d.1.get(0).unwrap().updated_at 
		},
		created_at: d.0.created_at,
		updated_at: d.0.updated_at
	}).collect();

	let total_page = ((query_count as f64 / PAGE_TAKE as f64) + 0.4).round() as i64;

	let pagination_response = ProductPaginate {
		data: query_search,
		paginate: PaginationResponse {
			per_page: PAGE_TAKE,
			total_page: total_page,
			count: query_count as i64,
			current_page: body.page
		}
	};

	(
		StatusCode::OK,
		json!(pagination_response).to_string()
	)
}

pub async fn create(
	State(db): State<DatabaseConnection>,
	Json(body): Json<ProductCreateDto>
) -> impl IntoResponse {
	let insert_data = product::ActiveModel {
		name: Set(body.name.to_owned()),
		description: Set(body.description.to_owned()),
		purchase_price: Set(body.purchase_price.to_owned()),
		selling_price: Set(body.selling_price.to_owned()),
		stock: Set(body.stock.to_owned()),
		discount: Set(body.discount.to_owned()),
		image: Set(body.image.to_owned()),
		category_id: Set(body.category_id.to_owned()),
		..Default::default()
	};

	match insert_data.insert(&db).await {
	    Ok(_) => {
	    	(StatusCode::ACCEPTED, json!({
	    		"success": true,
	    		"message": "Product Data was Created"
	    	}).to_string())
	    },
	    Err(e) => {
	    	(StatusCode::INTERNAL_SERVER_ERROR, json!({
	    		"success": false,
	    		"message": e.to_string()
	    	}).to_string())
	    }
	}
}

pub async fn update(
	State(db): State<DatabaseConnection>,
	Path(id): Path<i32>,
	Json(body): Json<ProductUpdateDto>
) -> impl IntoResponse {
	let updated_data = product::Entity::find_by_id(id).one(&db).await.unwrap();

	match updated_data {
		Some(val) => {
			let mut product_model: product::ActiveModel = val.into();

			product_model.name = Set(body.name.unwrap().to_owned());
			product_model.description = Set(body.description.unwrap().to_owned());
			product_model.purchase_price = Set(body.purchase_price.unwrap().to_owned());
			product_model.selling_price = Set(body.selling_price.unwrap().to_owned());
			product_model.stock = Set(body.stock.unwrap().to_owned());
			product_model.discount = Set(body.discount.unwrap().to_owned());
			product_model.image = Set(body.image.unwrap().to_owned());
			product_model.category_id = Set(body.category_id.unwrap().to_owned());

			match product_model.update(&db).await {
	    	    Ok(_) => (
		    		StatusCode::OK,
		    		json!({ "success": true, "message": "Product Data was Updated." }).to_string()
		    	),
	    	    Err(_) => (
	    	    	StatusCode::BAD_REQUEST,
	    	    	json!({ "success": false, "message": "Failed to Update Product Data." }).to_string()
	    	    )
			}
		},
		None => (
			StatusCode::NOT_FOUND,
			json!({
				"success": false,
				"message": "Data not Found!!!"
			}).to_string()
		)
	}
}

pub async fn delete(
	State(db): State<DatabaseConnection>,
	Path(id): Path<i32>
) -> impl IntoResponse {
	let query_delete_result = product::Entity::delete_by_id(id).exec(&db).await;

	match query_delete_result {
		Ok(val) => {
			if val.rows_affected == 0 {
				(
					StatusCode::NOT_FOUND,
					json!({ "success": false, "message": "Product Data Not Found!!!." }).to_string()
				)
			} else {
				(
					StatusCode::OK,
					json!({ "success": true, "message": "Product Data was Deleted." }).to_string()
				)
			}
		},
		Err(e) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	}
}