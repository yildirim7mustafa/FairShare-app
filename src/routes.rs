use mongodb::bson::doc;
use rocket::{get, post};
use rocket::http::Status;
use rocket::response::status;
use rocket_db_pools::{Connection};
use rocket::serde::json::{json, Json, Value};
use mongodb::bson::oid::ObjectId;
use crate::db::MainDatabase;
use crate::models::{Member, Organization};

#[get("/")]
pub fn index() -> Json<Value> {
    Json(json!({"success": true}))
}

#[post("/add_organization", data = "<data>", format = "json")]
pub async fn add_organization(
    db: Connection<MainDatabase>,
    data: Json<Organization>,
) -> status::Custom<Json<Value>> {
    if let Ok(res) = db
        .database("hesapla")
        .collection::<Organization>("organizations")
        .insert_one(data.into_inner(), None)
        .await
    {
        if let Some(id) = res.inserted_id.as_object_id() {
            return status::Custom(
                Status::Created,
                Json(json!({"status": "success", "message": format!("Organization ({}) created successfully", id.to_string())})),
            );
        }
    }
    status::Custom(
        Status::BadRequest,
        Json(json!({"status": "error", "message":"Organization could not be created"})),
    )

}

//add member
#[post("/add_member", data = "<data>", format = "json")]
pub async fn add_member(
    db: Connection<MainDatabase>,
    data: Json<Member>,
) -> status::Custom<Json<Value>> {
    let member = data.into_inner();

    // organization_id string -> ObjectId
    let org_oid = match ObjectId::parse_str(&member.organization_id) {
        Ok(oid) => oid,
        Err(_) => {
            return status::Custom(
                Status::BadRequest,
                Json(json!({
                    "status": "error",
                    "message": "invalid organization_id"
                })),
            )
        }
    };

    // 1. Üyeyi önce members koleksiyonuna ekle
    let coll_member = db
        .database("hesapla")
        .collection::<Member>("members");

    let insert_res = match coll_member.insert_one(&member, None).await {
        Ok(res) => res,
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to insert member: {}", e)
                })),
            )
        }
    };

    // MongoDB'nin oluşturduğu ID'yi al
    let inserted_id = match insert_res.inserted_id.as_object_id() {
        Some(id) => id,
        None => {
            return status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "status": "error",
                    "message": "Member inserted but no ObjectId returned"
                })),
            )
        }
    };

    // 2. Organization.members array’ine ekle
    let coll_org = db
        .database("hesapla")
        .collection::<Organization>("organizations");

    let update = doc! {
    "$push": { "members": { "id": inserted_id, "name": &member.name } }
    };

    let update_res = coll_org
        .update_one(doc! { "_id": org_oid }, update, None)
        .await;

    match update_res {
        Ok(res) if res.matched_count > 0 => status::Custom(
            Status::Ok,
            Json(json!({
                "status": "success",
                "message": "Member added successfully",
                "member_id": inserted_id.to_hex()
            })),
        ),
        Ok(_) => status::Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Organization not found"
            })),
        ),
        Err(err) => status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": format!("Database error: {}", err)
            })),
        ),
    }
}

