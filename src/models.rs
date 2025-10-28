use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};

fn current_time() -> DateTime{
    DateTime::now()
}
fn default_true() -> bool { true }


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Organization {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    #[serde(default = "current_time")]
    pub create_time: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Member {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub organization_id: ObjectId,
    pub name: String,

    #[serde(default = "default_true")]
    pub active: bool,

    #[serde(default = "current_time")]
    pub created_time: DateTime,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Expense {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub organization_id: ObjectId,
    pub title: String,
    pub amount: f64,
    pub paid_by: ObjectId,
    pub split_between: Vec<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(default = "current_time")]
    pub created_time: DateTime,

}