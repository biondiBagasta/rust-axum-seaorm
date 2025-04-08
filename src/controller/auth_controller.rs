use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
	extract::State, http::{ HeaderMap, StatusCode }, response::IntoResponse, Json
};

use bcrypt::{ hash, verify, DEFAULT_COST };

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter };
use serde_json::json;

use jsonwebtoken::{ encode, decode, DecodingKey, EncodingKey, Header, Validation };

use crate::model::auth_model::{ ChangePasswordBody, LoginBody };

use entity::user;

use crate::model::user_model::{ UserData, JwtClaims };

pub async fn login(
	State(db): State<DatabaseConnection>,
	Json(body): Json<LoginBody>
) -> impl IntoResponse {
	let query_find_first = user::Entity::find().filter(
		Condition::any().add(
			user::Column::Username.eq(&body.username)
		)
	).one(&db).await.unwrap();

	match query_find_first {
	    Some(val) => {
	    	let compared_password = verify(&body.password, &val.password).unwrap();

	    	if compared_password == false {
	    		(
	    			StatusCode::UNAUTHORIZED,
	    			json!({
	    				"success": false,
	    				"message": "INVALID USERNAME / PASSWORD"
	    			}).to_string()
	    		)
	    	} else {
			    let now = SystemTime::now()
		        .duration_since(UNIX_EPOCH)
		        .unwrap()
		        .as_secs();

		        let user_data = UserData {
		        	id: val.id,
		        	username: val.username,
		        	password: String::from(""),
		        	full_name: val.full_name,
		        	address: val.address,
		        	phone_number: val.phone_number,
		        	photo: val.photo,
		        	role: val.role,
		        	created_at: val.created_at,
		        	updated_at: val.updated_at
		        };

		        let jwt_claim = JwtClaims {
		        	user_data: user_data.clone(),
		        	exp: (now + 360000) as usize
		        };

	            dotenvy::dotenv().expect("Failed to load .env file.");

				let jwt_secret = std::env::var("JWT_SECRET").unwrap();

				let jwt_token = encode(&Header::default(), &jwt_claim, &EncodingKey::from_secret(jwt_secret.as_ref()))
				.expect("Failed to Create Token");

				(
					StatusCode::ACCEPTED,
					json!({ "success": true, "data": user_data.clone(), "token": jwt_token }).to_string()
				)
	    	}
	    },
	    None => (
			StatusCode::UNAUTHORIZED,
			json!({
				"success": false,
				"message": "INVALID USERNAME / PASSWORD"
			}).to_string()
		)
	}
}

pub async fn authenticated(
	headers: HeaderMap
) -> impl IntoResponse {
	let auth_header = headers.get("Authorization");

    dotenvy::dotenv().expect("Failed to load .env file.");

	let jwt_secret = std::env::var("JWT_SECRET").unwrap();

	match auth_header {
		Some(val) => {
			let header_value = val.to_str();

			match header_value {
				Ok(val2) => {
					let jwt_token = val2.strip_prefix("Bearer ");

					match jwt_token {
						Some(val3) => {
							let now = SystemTime::now()
							.duration_since(UNIX_EPOCH)
							.unwrap()
							.as_secs();

							let decoded_jwt = decode::<JwtClaims>(
								val3, 
								&DecodingKey::from_secret(jwt_secret.as_ref()), 
								&Validation::default()
							);

							match decoded_jwt {
							    Ok(val4) => {
							    	let jwt_claims = JwtClaims {
							    		user_data: UserData { 
							    			id: val4.claims.user_data.id, 
							    			username: val4.claims.user_data.username, 
							    			password: String::from(""), 
							    			full_name: val4.claims.user_data.full_name, 
							    			address: val4.claims.user_data.address, 
							    			phone_number: val4.claims.user_data.phone_number, 
							    			role: val4.claims.user_data.role, 
							    			photo: val4.claims.user_data.photo, 
							    			created_at: val4.claims.user_data.created_at, 
							    			updated_at: val4.claims.user_data.updated_at 
							    		},
							    		exp: (now + 36000000) as usize
							    	};


			    	    			let new_jwt_token = encode(&Header::default(), &jwt_claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
									.expect("Failed to Create Token");

									(
										StatusCode::OK,
										json!({ "success": true, "data": jwt_claims.user_data,  "token": new_jwt_token }).to_string()
									)
							    },
							    Err(_) => {
							    	(
										StatusCode::UNAUTHORIZED,
										json!({
											"success": false,
											"message": "INVALID CREDENTIALS."
										}).to_string()
									)
							    }
							}
						},
						None => (
							StatusCode::UNAUTHORIZED,
							json!({
								"success": false,
								"message": "Failed to Strip Bearer Prefix."
							}).to_string()
						)
					}
				},
				Err(_) => (
					StatusCode::UNAUTHORIZED,
					json!({
						"success": false,
						"message": "INVALID CREDENTIALS."
					}).to_string()
				)
			}
		},
		None =>  (
			StatusCode::UNAUTHORIZED,
			json!({
				"success": false,
				"message": "INVALID CREDENTIALS."
			}).to_string()
		)
	}
}

pub async fn change_password(
	State(db): State<DatabaseConnection>,
	Json(body): Json<ChangePasswordBody>
) -> impl IntoResponse {
	let query_find_first = user::Entity::find().filter(
		Condition::any().add(
			user::Column::Id.eq(body.id)
		)
	).one(&db).await;

	match query_find_first {
		Ok(result) => {
			match result {
			    Some(val) => {
			    	let compare_password = verify(body.old_password, &val.password);

			    	match compare_password {
			    		Ok(val2) => {
			    			if val2 == true {
			    				let new_password = hash(body.new_password, DEFAULT_COST).unwrap();

			    				let mut user_model: user::ActiveModel = val.into();

			    				user_model.password = Set(new_password.to_owned());
			    				user_model.updated_at = Set(chrono::Utc::now().naive_utc());

			    				match user_model.update(&db).await {
			    					Ok(_) => (
			    						StatusCode::OK,
			    						json!({ "success": true, "message": "Your password has been updated." }).to_string()
			    					),
			    					Err(_) => (
			    						StatusCode::BAD_REQUEST,
			    						json!({ "success": false, "message": "Request Failed." }).to_string()
			    					)
			    				}
			    			} else {
			    				(
			    					StatusCode::BAD_REQUEST,
			    					json!({ "success": false, "message": "Old password didn't matc!!!" }).to_string()
			    				)
			    			}
			    		},
			    		Err(_) => (
			    			StatusCode::BAD_REQUEST,
			    			json!({ "success": false, "message": "Failed during validation." }).to_string()
			    		)
			    	}
			    },
			    None => (
			    	StatusCode::NOT_FOUND,
			    	json!({ "success": false, "message": "Data User tidak ditemukan" }).to_string()
			    )
			}
		},
		Err(_) => (
			StatusCode::NOT_FOUND,
			json!({ "success": false, "message": "Data User tidak ditemukan." }).to_string()
		)
	}
}