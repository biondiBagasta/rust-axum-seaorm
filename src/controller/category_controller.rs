use axum::{
	extract::{ Path, State }, 
	http::StatusCode, 
	response::IntoResponse, Json
};

use sea_orm::{ActiveModelTrait, ActiveValue::Set, Condition, DatabaseConnection, EntityTrait, QueryOrder,
QueryFilter, ColumnTrait };
use serde_json::json;
use crate::model::category_model::{ CategoryCreateBody, CategoryData, CategoryUpdateBody };
use entity::category;

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
					json!({ "success": false, "message": "Category Data Not Found!!!." }).to_string()
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