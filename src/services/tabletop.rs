use super::{Db, Result};
use crate::models::Tabletop;
use crate::schema::tabletops::dsl::{id, name, tabletops, user_id};
use rocket::fairing::AdHoc;
use rocket::{get, post};
use rocket::{
    response::status::Created,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
pub struct NewTabletop {
    name: String,
    user_id: i32,
}

#[post("/", format = "json", data = "<req>")]
async fn create_tabletop(
    mut db: Connection<Db>,
    req: Json<NewTabletop>,
) -> Result<Created<Json<Tabletop>>> {
    let n = req.name.to_string();
    let uid = req.user_id;
    // TODO name validation
    let result = diesel::insert_into(tabletops)
        .values((name.eq(&n), user_id.eq(&uid)))
        .returning(Tabletop::as_returning())
        .get_result(&mut db)
        .await?;
    Ok(Created::new("/").body(Json(result)))
}

#[get("/user/<uid>")]
async fn get_tabletops(uid: i32, mut db: Connection<Db>) -> Result<Json<Vec<Tabletop>>> {
    // TODO this probably should be dereferenced by going through the players for a user
    let results = tabletops
        .filter(user_id.eq(&uid))
        .select(Tabletop::as_select())
        .load(&mut db)
        .await?;
    Ok(Json(results))
}

#[get("/<ttid>")]
async fn get_tabletop(mut db: Connection<Db>, ttid: i32) -> Result<Json<Tabletop>> {
    let result = tabletops
        .filter(id.eq(&ttid))
        .select(Tabletop::as_select())
        .first(&mut db)
        .await?;
    Ok(Json(result))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("tabletop routes", |rocket| async {
        rocket.mount(
            "/tabletops",
            rocket::routes![create_tabletop, get_tabletops, get_tabletop],
        )
    })
}
