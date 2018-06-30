use diesel::prelude::*;
use models::Move;
use rocket::http::Status;
use rocket_contrib::Json;
use utils::establish_connection;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/root_moves")]
pub fn root_moves() -> Json<Vec<Move>> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let results = moves
        .filter(parent.is_null())
        .load::<Move>(&connection)
        .expect("Error loading posts");
    Json(results)
}

#[put("/<move_id>", format = "application/json", data = "<move_information>")]
pub fn update_move(move_id: i32, move_information: Json<Move>) -> Result<(), Status> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let mut move_vec = moves
        .find(move_id)
        .load::<Move>(&connection)
        .map_err(|_| Status::InternalServerError)?;

    let actual_move = move_vec.pop().ok_or_else(|| Status::NotFound)?;
    ::diesel::update(&actual_move).set(move_information.0);
    Ok(())
}
#[post("/", format = "application/json", data = "<move_information>")]
pub fn create_move(move_information: Json<Move>) -> Result<(), Status> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let mut move_vec = moves
        .find(move_id)
        .load::<Move>(&connection)
        .map_err(|_| Status::InternalServerError)?;

    let actual_move = move_vec.pop().ok_or_else(|| Status::NotFound)?;
    ::diesel::update(&actual_move).set(move_information.0);
    Ok(())
}

#[get("/<move_id>")]
pub fn request_move(move_id: i32) -> Result<Json<(Move, Vec<Move>)>, Status> {
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
        Err(Status::NoContent)
    }
}
