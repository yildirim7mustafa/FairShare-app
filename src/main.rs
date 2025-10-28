
use rocket::{launch, routes};
use rocket_db_pools::Database;

mod db;
mod routes;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::MainDatabase::init()).mount(
        "/",
        routes![
            routes::index,
            routes::add_organization,
            routes::add_member
        ],
    )
}