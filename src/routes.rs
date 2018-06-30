use diesel::prelude::*;
use models::Move;
use rocket::http::Status;
use rocket::response::{Failure, NamedFile};
use rocket_contrib::Json;
use utils::{establish_connection, MoveForm};

#[get("/")]
fn index() -> ::std::io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/")]
pub fn root_moves() -> Result<Json<Vec<Move>>, Failure> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let results = moves
        .filter(parent.is_null())
        .load::<Move>(&connection)
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(results))
}

#[post("/", format = "application/json", data = "<move_information>")]
pub fn create_move(move_information: Json<MoveForm>) -> Result<(), Failure> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    ::diesel::insert_into(moves)
        .values(move_information.0)
        .execute(&connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(())
}

#[get("/<move_id>")]
pub fn request_move(move_id: i32) -> Result<Json<(Move, Vec<Move>)>, Failure> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let mut parent_move_vec = moves
        .find(move_id)
        .load::<Move>(&connection)
        .map_err(|_| Status::InternalServerError)?;

    if let Some(parent_move) = parent_move_vec.pop() {
        let children = moves
            .filter(parent.eq(parent_move.id))
            .load::<Move>(&connection)
            .map_err(|_| Status::InternalServerError)?;

        Ok(Json((parent_move, children)))
    } else {
        Err(Failure(Status::NoContent))
    }
}

#[put("/<move_id>", format = "application/json", data = "<move_information>")]
pub fn update_move(move_id: i32, move_information: Json<Move>) -> Result<(), Failure> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let n = ::diesel::update(moves)
        .filter(id.eq(move_id))
        .set(move_information.0)
        .execute(&connection)
        .map_err(|_| Status::InternalServerError)?;

    if n != 1 {
        Err(Failure(Status::NoContent))
    } else {
        Ok(())
    }
}

#[delete("/<move_id>")]
pub fn delete_move(move_id: i32) -> Result<(), Failure> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let n = ::diesel::delete(moves.find(move_id))
        .execute(&connection)
        .map_err(|_| Status::InternalServerError)?;

    if n != 1 {
        Err(Failure(Status::NoContent))
    } else {
        Ok(())
    }
}
