use std::convert::TryFrom;
use actix_web::{post, HttpResponse, web::{Data, Json}};
use crate::models::owner_model::{Owner, OwnerRequest};
use crate::services::db::Database;

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone(),
            })
        .expect("Error converting DogRequest to Dog."),
    )
        .await
    {
        Ok(owner) => HttpResponse::Ok().json(owner),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}