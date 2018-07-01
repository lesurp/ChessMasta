use models::*;
use rocket::response::Failure;
use rocket_contrib::Json;
use utils::MoveForm;

#[get("/")]
pub fn root_moves() -> Result<Json<Vec<Move>>, Failure> {
    Move::get_root_moves().map(|r| Json(r))
}

#[post("/", format = "application/json", data = "<move_information>")]
pub fn create_move(move_information: Json<MoveForm>) -> Result<(), Failure> {
    Move::create_move(move_information.0)
}

#[get("/<move_id>")]
pub fn request_move(move_id: i32) -> Result<Json<(Move, Vec<Move>)>, Failure> {
    Move::get_move(move_id).map(|r| Json(r))
}

#[put("/", format = "application/json", data = "<move_information>")]
pub fn update_move(move_information: Json<Move>) -> Result<(), Failure> {
    Move::update_move(move_information.0)
}

#[delete("/<move_id>")]
pub fn delete_move(move_id: i32) -> Result<(), Failure> {
    Move::delete_move(move_id)
}
