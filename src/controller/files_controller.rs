use axum:: {
	extract::{ Multipart, Path },
	http::{ StatusCode },
	response::{ IntoResponse, Response }
};
use reqwest::header;
use serde_json::json;
use std::{ fs, path::PathBuf };
use tokio::{ fs::File, io::{ AsyncReadExt, AsyncWriteExt } };

// User Files Controller
pub async fn upload_user_image(mut multipart: Multipart) -> impl IntoResponse {
	let upload_dir = "uploads/user";

	fs::create_dir_all(upload_dir).unwrap();

	let field = multipart.next_field().await.unwrap();

	match field {
		Some(mut val) => {
			let file_name = val.file_name().map(|name| name.to_string());

			match file_name {
				Some(val2) => {
					let ext = val2.split(".").last().unwrap_or("");
					let timestamp = chrono::Utc::now().timestamp();

					let new_file_name = if ext == "jfif" {
						format!("{timestamp}_{}", val2.split(".").next().unwrap())
					} else {
						format!("{timestamp}_{val2}")
					};

					let file_path = format!("{}/{}", upload_dir, new_file_name);

					let mut file = File::create(&file_path).await.unwrap();

					while let Some(chunk) = val.chunk().await.unwrap() {
						file.write_all(&chunk).await.unwrap();
					}

					(
						StatusCode::OK,
						json!({ "file_name": new_file_name, "file_extension": ext }).to_string()
					)
				},
				None => (
					StatusCode::BAD_REQUEST,
					"Something wrong happened".to_string()
				)
			}
		},
		None => (
			StatusCode::BAD_REQUEST,
			"No File to Upload!!!".to_string()
		)
	}
}

pub async fn get_user_image(Path(filename): Path<String>) -> Response {
	let file_path = PathBuf::from(format!("uploads/user/{}", filename));

	if file_path.exists() {
		match File::open(&file_path).await {
		    Ok(mut file) => {
		    	let mut contents = Vec::new();

		    	if file.read_to_end(&mut contents).await.is_ok() {
		    		return Response::builder()
		    		.status(StatusCode::OK)
		    		.header(header::CONTENT_TYPE, "image/*")
		    		.body(axum::body::Body::from(contents))
		    		.unwrap();
		    	}
		    },
		    Err(_) => {}
		}
	}

	(StatusCode::NOT_FOUND, "File Not Found.").into_response()
}

pub async fn delete_user_image(Path(filename): Path<String>) -> impl IntoResponse {
	let file_path = format!("uploads/user/{}", filename);

	if filename != "default_user.png" && std::path::Path::new(&file_path).exists() {
		fs::remove_file(file_path).unwrap();

		(StatusCode::OK, "Files was Deleted.")
	} else {
		(StatusCode::NOT_FOUND, "File not found or cannot deleted the default file.")
	}
}

// Product Files Controller
pub async fn upload_product_image(mut multipart: Multipart) -> impl IntoResponse {
	let upload_dir = "uploads/product";

	fs::create_dir_all(upload_dir).unwrap();

	let field = multipart.next_field().await.unwrap();

	match field {
		Some(mut val) => {
			let file_name = val.file_name().map(|name| name.to_string());

			match file_name {
				Some(val2) => {
					let ext = val2.split(".").last().unwrap_or("");
					let timestamp = chrono::Utc::now().timestamp();

					let new_file_name = if ext == "jfif" {
						format!("{timestamp}_{}", val2.split(".").next().unwrap())
					} else {
						format!("{timestamp}_{val2}")
					};

					let file_path = format!("{}/{}", upload_dir, new_file_name);

					let mut file = File::create(&file_path).await.unwrap();

					while let Some(chunk) = val.chunk().await.unwrap() {
						file.write_all(&chunk).await.unwrap();
					}

					(
						StatusCode::OK,
						json!({ "file_name": new_file_name, "file_extension": ext }).to_string()
					)
				},
				None => (
					StatusCode::BAD_REQUEST,
					"Something wrong happened".to_string()
				)
			}
		},
		None => (
			StatusCode::BAD_REQUEST,
			"No File to Upload!!!".to_string()
		)
	}
}

pub async fn get_product_image(Path(filename): Path<String>) -> Response {
	let file_path = PathBuf::from(format!("uploads/product/{}", filename));

	if file_path.exists() {
		match File::open(&file_path).await {
		    Ok(mut file) => {
		    	let mut contents = Vec::new();

		    	if file.read_to_end(&mut contents).await.is_ok() {
		    		return Response::builder()
		    		.status(StatusCode::OK)
		    		.header(header::CONTENT_TYPE, "image/*")
		    		.body(axum::body::Body::from(contents))
		    		.unwrap();
		    	}
		    },
		    Err(_) => {}
		}
	}

	(StatusCode::NOT_FOUND, "File Not Found.").into_response()
}

pub async fn delete_product_image(Path(filename): Path<String>) -> impl IntoResponse {
	let file_path = format!("uploads/product/{}", filename);

	if filename != "default_user.png" && std::path::Path::new(&file_path).exists() {
		fs::remove_file(file_path).unwrap();

		(StatusCode::OK, "Files was Deleted.")
	} else {
		(StatusCode::NOT_FOUND, "File not found or cannot deleted the default file.")
	}
}