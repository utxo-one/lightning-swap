use sha2::{Sha256, Digest};

use rocket::serde::json::Json;
use crate::models::swap_model::{NewSwap, Swap};
use crate::repositories::swap_repository::{store_swap, show_swap, delete_swap, index_swap};

use::rocket::{get, post, delete};

#[get("/")]
pub fn index() -> Json<Vec<Swap>> {
    Json(index_swap())
}

#[post("/")]
pub fn store() -> Json<Swap> {

    let mut hasher = Sha256::new();
    hasher.update(&chrono::Utc::now().timestamp_millis().to_be_bytes());
    let uuid = format!("{:x}", hasher.finalize());

    let new_swap = NewSwap {
        
        uuid: uuid,
        amount: 100,
        payment_request: "uuid,".to_string(),
        txid: None,
        status: "pending".to_string(),
    };

    Json(store_swap(new_swap))
}

#[get("/<swap_uuid>")]
pub fn show(swap_uuid: String) -> Json<Swap> {
    Json(show_swap(swap_uuid))
}

#[delete("/<swap_uuid>")]
pub fn delete(swap_uuid: String) -> Json<Swap> {
    Json(delete_swap(swap_uuid))
}