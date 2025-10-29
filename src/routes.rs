use crate::db::MainDatabase;
use crate::models::{Expense, Member, Organization};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value, json};
use rocket::{delete, get, post};
use rocket_db_pools::Connection;

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
                Json(
                    json!({"status": "success", "message": format!("Organization ({}) created successfully", id.to_string())}),
                ),
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
    let org_oid = member.organization_id.clone();

    let org_coll = db
        .database("hesapla")
        .collection::<Organization>("organizations");
    let member_coll = db.database("hesapla").collection::<Member>("members");

    // 1. Organization var mı?
    let org_result = org_coll.find_one(doc! { "_id": &org_oid }, None).await;
    if org_result.is_err() {
        return status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": "Database lookup failed for organization"
            })),
        );
    }

    if org_result.unwrap().is_none() {
        return status::Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Organization not found"
            })),
        );
    }

    // 2. Organization varsa, yeni member ekle
    let insert_result = member_coll.insert_one(&member, None).await;
    if let Err(err) = insert_result {
        return status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": format!("Database insert error: {}", err)
            })),
        );
    }

    // 3. Başarılıysa success dön
    status::Custom(
        Status::Created,
        Json(json!({
            "status": "success",
            "message": "Member added successfully"
        })),
    )
}

#[post("/add_expense", data = "<data>", format = "json")]
pub async fn add_expense(
    db: Connection<MainDatabase>,
    data: Json<Expense>,
) -> status::Custom<Json<Value>> {
    let expense = data.into_inner();

    let org_oid = expense.organization_id.clone();
    let member_oid = expense.paid_by.clone();

    let org_coll = db
        .database("hesapla")
        .collection::<Organization>("organizations");
    let member_coll = db.database("hesapla").collection::<Member>("members");
    let expense_coll = db.database("hesapla").collection::<Expense>("expenses");

    // 1. Organizasyon var mı?
    if org_coll
        .find_one(doc! { "_id": &org_oid }, None)
        .await
        .unwrap_or(None)
        .is_none()
    {
        return status::Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Organization not found"
            })),
        );
    }

    // 2. paid_by üyesi var mı?
    if member_coll
        .find_one(doc! { "_id": &member_oid }, None)
        .await
        .unwrap_or(None)
        .is_none()
    {
        return status::Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Paid_by member not found"
            })),
        );
    }

    // 3. split_between üyeleri var mı?
    for member_id in &expense.split_between {
        if member_coll
            .find_one(doc! { "_id": member_id }, None)
            .await
            .unwrap_or(None)
            .is_none()
        {
            return status::Custom(
                Status::NotFound,
                Json(json!({
                    "status": "error",
                    "message": format!("Member in split_between not found: {}", member_id)
                })),
            );
        }
    }

    // 4. Hepsi doğruysa expense kaydet
    match expense_coll.insert_one(&expense, None).await {
        Ok(_) => status::Custom(
            Status::Created,
            Json(json!({
                "status": "success",
                "message": "Expense added successfully"
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

#[delete("/delete_expense/<expense_id>")]
pub async fn delete_expense(
    db: Connection<MainDatabase>,
    expense_id: &str,
) -> status::Custom<Json<Value>> {
    let obj_id = match ObjectId::parse_str(expense_id) {
        Ok(oid) => oid,
        Err(_) => {
            return status::Custom(
                Status::BadRequest,
                Json(json!({
                    "status": "error",
                    "message": "Invalid expense_id format"
                })),
            );
        }
    };

    let expense_coll = db.database("hesapla").collection::<Expense>("expenses");

    match expense_coll.delete_one(doc! { "_id": obj_id }, None).await {
        Ok(res) if res.deleted_count == 1 => status::Custom(
            Status::Ok,
            Json(json!({
                "status": "success",
                "message": "Expense deleted successfully"
            })),
        ),
        Ok(_) => status::Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Expense not found"
            })),
        ),
        Err(err) => status::Custom(
            Status::InternalServerError,
            Json(json!({
                "message": format!("Database delete error: {}", err)
            })),
        ),
    }
}
