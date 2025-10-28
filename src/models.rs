use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};

fn current_time() -> DateTime{
    DateTime::now()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MemberRef {
    pub id: ObjectId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Organization {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    #[serde(default)]
    pub members: Vec<MemberRef>,

    #[serde(default = "current_time")]
    pub create_time: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Member {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub organization_id: String,
    pub name: String,


    #[serde(default = "current_time")]
    pub created_time: DateTime,

    #[serde(default = "current_time")]
    pub updated_time: DateTime,
}
