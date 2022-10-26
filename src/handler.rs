use hyper::{ Body, Method, Request, Response, StatusCode};
use std::error::Error;

use crate::services::*;


pub async fn cars_handler(
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let path = req.uri().path().to_owned();
    let path_segments = path.split("/").collect::<Vec<&str>>();
    let base_path = path_segments[1];

    match (req.method(), base_path) {
        (&Method::GET, "cars") => {
            if path_segments.len() <= 2 {
                let res = get_car_list();
                return Ok(res);
            }

            let car_id = path_segments[2];

            if car_id.trim().is_empty() {
                let res = get_car_list();
                return Ok(res);
            } else {
                // code to fill whenever path is /cars/:id
                let res = get_car_by_id(&car_id.to_string());
                Ok(res)
            }
        }

        (&Method::POST, "cars") => create_car(req).await,

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}