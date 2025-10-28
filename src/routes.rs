
use mongodb::bson::doc;
use rocket::{get, post};
use rocket::http::Status;
use rocket::response::status;
use rocket_db_pools::{Connection};
use rocket::serde::json::{json, Json, Value};
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
    let org_oid = member.organization_id.clone();
    let org_coll = db
        .database("hesapla")
        .collection::<Organization>("organizations");

    match org_coll.find_one(doc! {"_id": &org_oid}, None).await {
        Ok(Some(_)) => {
            let member_coll = db
                .database("hesapla")
                .collection::<Member>("members");

            match member_coll.insert_one(&member, None).await {
                Ok(_) => status::Custom(
                    Status::Created,
                    Json(json!({
                        "status": "success",
                        "message": "Member added successfully"
                    })),
                ),
                Err(err) => status::Custom(
                    Status::InternalServerError,
                    Json(json!({
                        "status": "error",
                        "message": format!("Database insert error: {}", err)
                    })),
                ),
            }



        }
        Ok(None) => status::Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Organization not found"
            })),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": "Database lookup failed"
            })),
        ),
    }
}