use std::time::SystemTime;
use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dog {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DogRequest {
    pub owner: String,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: DogRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).expect("Must be a valid ObjectId"),
            name: item.name,
            age: item.age,
            breed: item.breed,
        })
    }
}