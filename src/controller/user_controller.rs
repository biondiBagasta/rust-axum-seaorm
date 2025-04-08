use axum::{
	extract::{ Path, State },
	http::StatusCode,
	response::IntoResponse,
	Json
};

use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{
	ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait,
	QueryOrder,
};

use serde_json::json;

use crate::model::user_model::{ UserCreateBody, UserData, UserUpdateBody };

use entity::user;

pub async fn find_many(
	State(db): State<DatabaseConnection>
) -> impl IntoResponse {
	let query_find_many: Vec<UserData> = user::Entity::find().order_by_asc(
		user::Column::FullName
	).all(&db).await.unwrap().iter().into_iter().map(|d| UserData {
		id: d.id,
		username: d.username.clone(),
		password: "".to_string(),
		full_name: d.full_name.clone(),
		address: d.address.clone(),
		phone_number: d.phone_number.clone(),
		role: d.role.clone(),
		photo: d.photo.clone(),
		created_at: d.created_at,
		updated_at: d.updated_at
	}).collect();

	(StatusCode::OK, json!(query_find_many).to_string())
}

pub async fn create(
	State(db): State<DatabaseConnection>,
	Json(body): Json<UserCreateBody>
) -> impl IntoResponse {
	let hashed_password = hash(body.password, DEFAULT_COST).unwrap();

	let data = user::ActiveModel {
		username: Set(body.username),
		password: Set(hashed_password),
		full_name: Set(body.full_name),
		address: Set(body.address),
		phone_number: Set(body.phone_number),
		role: Set(body.role),
		photo: Set(body.photo),
		..Default::default()
	};

	match data.insert(&db).await {
	    Ok(_) => {
	    	(StatusCode::ACCEPTED, json!({
	    		"success": true,
	    		"message": "User Data was Created"
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
	Json(body): Json<UserUpdateBody>
) -> impl IntoResponse {

	let updated_data = user::Entity::find_by_id(id).one(&db).await.unwrap();

	match updated_data {
		Some(val) => {
			let compare_password = verify(&body.password.as_ref().unwrap(), &val.password).unwrap();

			let mut user_model: user::ActiveModel = val.into();

			if compare_password == true {
				user_model.username = Set(body.username.unwrap()).to_owned();
				user_model.full_name = Set(body.full_name.unwrap()).to_owned();
				user_model.address = Set(body.address.unwrap()).to_owned();
				user_model.phone_number = Set(body.phone_number.unwrap()).to_owned();
				user_model.role = Set(body.role.unwrap()).to_owned();
				user_model.photo = Set(body.photo.unwrap()).to_owned();
				user_model.updated_at = Set(chrono::Utc::now().naive_utc());

				match user_model.update(&db).await {
    	    	    Ok(_) => (
			    		StatusCode::OK,
			    		json!({ "success": true, "message": "User Data was Updated." }).to_string()
			    	),
		    	    Err(_) => (
		    	    	StatusCode::BAD_REQUEST,
		    	    	json!({ "success": false, "message": "Failed to Update User Data." }).to_string()
		    	    )
				}
			} else {
				let hashed_new_password = hash(&body.password.unwrap(), DEFAULT_COST).unwrap();

				user_model.username = Set(body.username.unwrap()).to_owned();
				user_model.full_name = Set(body.full_name.unwrap()).to_owned();
				user_model.address = Set(body.address.unwrap()).to_owned();
				user_model.phone_number = Set(body.phone_number.unwrap()).to_owned();
				user_model.role = Set(body.role.unwrap()).to_owned();
				user_model.photo = Set(body.photo.unwrap()).to_owned();
				user_model.updated_at = Set(chrono::Utc::now().naive_utc()); 
				user_model.password = Set(hashed_new_password).to_owned();

				match user_model.update(&db).await {
    	    	    Ok(_) => (
			    		StatusCode::OK,
			    		json!({ "success": true, "message": "User Data was Updated." }).to_string()
			    	),
		    	    Err(_) => (
		    	    	StatusCode::BAD_REQUEST,
		    	    	json!({ "success": false, "message": "Failed to Update User Data." }).to_string()
		    	    )
				}
			}
		},
		None => (
			StatusCode::NOT_FOUND,
			json!({ "success": false, "message": "Data not Found!!!"}).to_string()
		)
	}
}

pub async fn delete(
	State(db): State<DatabaseConnection>,
	Path(id): Path<i32>
) -> impl IntoResponse {
	let query_delete_result = user::Entity::delete_by_id(id).exec(&db).await;

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
					json!({ "success": true, "message": "User Data was Deleted." }).to_string()
				)
			}
		},
		Err(e) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	}
}