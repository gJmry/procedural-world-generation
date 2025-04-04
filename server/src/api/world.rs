use crate::generators::world_generator::{generate_world, serialize_to_json};
use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldRequest {
    pub height: Option<u16>,
    pub width: Option<u16>
}

#[post("/world")]
pub async fn world(world_request: Json<WorldRequest>) -> HttpResponse {
    if let (Some(height), Some(width)) = (world_request.height, world_request.width) {
        let world_data = generate_world(height, width);
        let serialized_world = serialize_to_json(world_data);

        HttpResponse::Ok().json(serialized_world)

    } else {
        HttpResponse::Ok().body("Height or width is null")
    }
}