use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::Failure;
use schema::moves;
use schema::moves::dsl::*;
use utils::establish_connection;

#[derive(Queryable, Debug, AsChangeset, Identifiable, Serialize, Deserialize)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "moves"]
pub struct Move {
    pub id: i32,
    pub parent: Option<i32>,
    pub turn: i32,
    pub name_: String,
    pub special_name: Option<String>,
    pub line_description: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "moves"]
pub struct NewMove {
    pub parent: Option<i32>,
    pub name_: String,
    pub special_name: Option<String>,
    pub line_description: Option<String>,
}

impl Move {
    pub fn get_move(move_id: i32) -> Result<(Move, Vec<Move>), Failure> {
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

            Ok((parent_move, children))
        } else {
            Err(Failure(Status::NoContent))
        }
    }

    pub fn get_move_with_parent(move_id: i32) -> Result<(Option<Move>, Move, Vec<Move>), Failure> {
        let connection = establish_connection();
        let parent_move = Move::request_move_parent(move_id)?;
        let mut current_move_vec = moves
            .find(move_id)
            .load::<Move>(&connection)
            .map_err(|_| Status::InternalServerError)?;

        if let Some(current_move) = current_move_vec.pop() {
            let children = moves
                .filter(parent.eq(current_move.id))
                .load::<Move>(&connection)
                .map_err(|_| Status::InternalServerError)?;

            Ok((parent_move, current_move, children))
        } else {
            Err(Failure(Status::NoContent))
        }
    }

    pub fn get_root_moves() -> Result<Vec<Move>, Failure> {
        let connection = establish_connection();
        let results = moves
            .filter(parent.is_null())
            .load::<Move>(&connection)
            .map_err(|_| Status::InternalServerError)?;
        Ok(results)
    }

    pub fn create_move(move_information: NewMove) -> Result<i32, Failure> {
        let connection = establish_connection();

        let turn_value = match move_information.parent {
            None => 0 as i32,
            Some(parent_id) => match Move::request_single_move(parent_id)? {
                Some(parent_move) => parent_move.turn + 1,
                None => return Err(Failure(Status::NoContent)),
            },
        };

        ::diesel::insert_into(moves)
            .values((move_information, turn.eq(turn_value)))
            .execute(&connection)
            .map_err(|_| Status::InternalServerError)?;

        moves
            .order(id.desc())
            .select(id)
            .first::<i32>(&connection)
            .map_err(|_| Failure(Status::InternalServerError))
    }

    pub fn update_move(move_information: Move) -> Result<(), Failure> {
        let connection = establish_connection();
        let n = ::diesel::update(&move_information)
            .set((
                moves::name_.eq(&move_information.name_),
                moves::special_name.eq(&move_information.special_name),
                moves::line_description.eq(&move_information.line_description),
            ))
            .execute(&connection)
            .map_err(|_| Status::InternalServerError)?;

        if n != 1 {
            Err(Failure(Status::NoContent))
        } else {
            Ok(())
        }
    }

    pub fn delete_move(move_id: i32) -> Result<(), Failure> {
        let connection = establish_connection();
        match Move::request_single_move(move_id)? {
            None => Err(Failure(Status::NoContent)),
            Some(move_) => Move::delete_move_recursive(move_, &connection),
        }
    }

    fn delete_move_recursive(move_: Move, connection: &SqliteConnection) -> Result<(), Failure> {
        let children = moves
            .filter(parent.eq(move_.id))
            .load::<Move>(connection)
            .map_err(|_| Status::InternalServerError)?;

        for child in children {
            Move::delete_move_recursive(child, connection)?;
        }

        ::diesel::delete(&move_)
            .execute(connection)
            .map_err(|_| Status::InternalServerError)?;
        Ok(())
    }

    pub fn request_single_move(move_id: i32) -> Result<Option<Move>, Failure> {
        let connection = establish_connection();
        let mut move_info = moves
            .find(move_id)
            .load::<Move>(&connection)
            .map_err(|_| Status::InternalServerError)?;
        Ok(move_info.pop())
    }

    pub fn request_move_parent(move_id: i32) -> Result<Option<Move>, Failure> {
        let connection = establish_connection();
        let mut child_move = moves
            .find(move_id)
            .load::<Move>(&connection)
            .map_err(|_| Status::InternalServerError)?;

        if let Some(child_move) = child_move.pop() {
            match child_move.parent {
                Some(parent_id) => match Move::request_single_move(parent_id)? {
                    None => Err(Failure(Status::InternalServerError)),
                    Some(parent_move) => Ok(Some(parent_move)),
                },
                None => Ok(None),
            }
        } else {
            return Err(Failure(Status::NoContent));
        }
    }
}
