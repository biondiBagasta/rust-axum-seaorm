use axum::{
	extract::{ Path, State }, 
	http::StatusCode, 
	response::IntoResponse, Json
};

use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::sea_query::Expr;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, Condition, DatabaseConnection, EntityTrait, QueryOrder,
QueryFilter, ColumnTrait, PaginatorTrait, QuerySelect };
use serde_json::json;
use crate::model::{category_model::{ CategoryCreateBody, CategoryData, CategoryUpdateBody, CategoryPaginate }, pagination_model::PaginationBody};
use entity::category;
use crate::model::pagination_model::PaginationResponse;

pub async fn find_many(State(db): State<DatabaseConnection>) -> impl IntoResponse {
	let query_data: Vec<CategoryData> = category::Entity::find()
	.order_by_asc(category::Column::Name).all(&db).await.unwrap().iter().into_iter().map(|d| CategoryData {
		id: d.id,
		name: d.name.clone(),
		created_at: d.created_at,
		updated_at: d.updated_at
	}).collect();

	(
		StatusCode::OK,
		json!(query_data).to_string()
	)
}

pub async fn search_paginate(
	State(db): State<DatabaseConnection>,
	Json(body): Json<PaginationBody>
) -> impl IntoResponse {
	const PAGE_TAKE: i64 = 10;

	let page_offset = (&body.page - 1) * PAGE_TAKE;

	let query_count = category::Entity::find().filter(
		Condition::any().add(
			category::Column::Name.contains(&body.term)
		)
	).count(&db).await.unwrap();

	let query_search: Vec<CategoryData> = category::Entity::find().filter(
		Condition::any().add(
			Expr::col(category::Column::Name).ilike(format!("%{}%", body.term))
		)
	).order_by_asc(category::Column::Name).offset(page_offset as u64).limit(PAGE_TAKE as u64).all(&db).await.unwrap().iter().into_iter().map(|d| CategoryData {
		id: d.id,
		name: d.name.clone(),
		created_at: d.created_at,
		updated_at: d.updated_at
	}).collect();

	let total_page = ((query_count as f64 / PAGE_TAKE as f64) + 0.4).round() as i64;

	let pagination_response = CategoryPaginate {
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

pub async fn find_first(State(db): State<DatabaseConnection>, Path(id): Path<i32>) -> impl IntoResponse {
	let query_find_first = category::Entity::find().filter(
		Condition::all().add(
			category::Column::Id.eq(id)
		)
	).one(&db).await.unwrap();

	match query_find_first {
		Some(val) => {
			let category_data = CategoryData {
				id: val.id,
				name: val.name,
				created_at: val.created_at,
				updated_at: val.updated_at
			};

			(
				StatusCode::OK,
				json!(category_data).to_string()
			)
		},
		None => (
			StatusCode::NOT_FOUND,
			json!({ "message": "Data Not Found!!!" }).to_string()
		)
	}
}

pub async fn create(State(db): State<DatabaseConnection>,
	Json(body): Json<CategoryCreateBody>
) -> impl IntoResponse {
	let category_data = category::ActiveModel {
		name: Set(body.name.to_owned()),
		..Default::default()
	};

	match category_data.insert(&db).await {
	    Ok(_) => {
	    	(StatusCode::ACCEPTED, json!({
	    		"success": true,
	    		"message": "Category Data was Created"
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

pub async fn update(State(db): State<DatabaseConnection>,
	Path(id): Path<i32>, Json(body): Json<CategoryUpdateBody>
) -> impl IntoResponse {
	let update_category_data = category::Entity::find_by_id(id).one(&db).await.unwrap();

	match update_category_data {
	    Some(val) => {
	    	let mut category_model: category::ActiveModel = val.into();

	    	category_model.name = Set(body.name.unwrap().to_owned());
	    	category_model.updated_at = Set(chrono::Utc::now().naive_utc()); // I want to use Current NaiveDateTime

	    	match category_model.update(&db).await {
	    	    Ok(_) => (
		    		StatusCode::OK,
		    		json!({ "success": true, "message": "Category Data was Updated." }).to_string()
		    	),
	    	    Err(_) => (
	    	    	StatusCode::BAD_REQUEST,
	    	    	json!({ "success": false, "message": "Failed to Update Category Data." }).to_string()
	    	    )
	    	}
	    },
	    None => (
	    	StatusCode::NOT_FOUND,
	    	json!({ "success": false, "message": "Data Not Found!!!"}).to_string()
	    )
	}
}

pub async fn delete(State(db): State<DatabaseConnection>,
	Path(id): Path<i32>
) -> impl IntoResponse {
	let query_delete_result = category::Entity::delete_by_id(id).exec(&db).await;

	match query_delete_result {
		Ok(val) => {
			if val.rows_affected == 0 {
				(
					StatusCode::NOT_FOUND,
					json!({ "success": false, "message": "Data Not Found!!!." }).to_string()
				)
			} else {
				(
					StatusCode::OK,
					json!({ "success": true, "message": "Category Data was Deleted." }).to_string()
				)
			}
		},
		Err(e) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	}
}