use axum::{
	body::Body,
	http::{ Request, StatusCode },
	middleware::Next,
	response::{Response}
};

use jsonwebtoken::{ decode, DecodingKey, Validation };
use serde_json::json;
use crate::model::user_model::JwtClaims;
use crate::utils::utils::JWT_SECRET;

pub async fn auth_guard(req: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {
	let extracted_header_value = req.headers().get("Authorization");

	match extracted_header_value {
		Some(header_val) => {
			let extracted_header_value_string = header_val.to_str();

			match extracted_header_value_string {
				Ok(header_val_str) => {
					let jwt_token = header_val_str.strip_prefix("Bearer ");

					match jwt_token {
						Some(token) => {
							let decoed_token = decode::<JwtClaims>(
								token, 
								&DecodingKey::from_secret(JWT_SECRET.as_ref()), 
								&Validation::default()
							);

							match decoed_token {
								Ok(_) => Ok(
									next.run(req).await
								),
								Err(e) => Err(
									(
										StatusCode::INTERNAL_SERVER_ERROR,
										json!({ "success": false, "message": e.to_string() }).to_string()
									)
								)
							}
						},
						None => Err(
							(
								StatusCode::UNAUTHORIZED,
								json!({ "success": false, "message": "Unauthorized" }).to_string()
							)
						)
					}
				},
				Err(_) => Err(
					(
						StatusCode::UNAUTHORIZED,
						json!({ "success": false, "message": "Unauthorized" }).to_string()
					)
				)
			}
		},
		None => Err(
			(
				StatusCode::UNAUTHORIZED,
				json!({ "success": false, "message": "Unauthorized" }).to_string()
			)
		)
	}
}