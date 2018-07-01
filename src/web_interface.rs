use models::Move;
use rocket::http::Status;
use rocket::response::{Failure, NamedFile, Redirect};
use rocket_contrib::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct GetMoveContext {
    parent_move: Move,
    move_list: Vec<Move>,
}

#[get("/")]
fn index() -> Result<Template, Failure> {
    let root_moves = Move::get_root_moves()?;

    let mut context = HashMap::new();
    context.insert("move_list", root_moves);
    Ok(Template::render("index", &context))
}

#[get("/<move_id>")]
fn get_move(move_id: i32) -> Result<Template, Failure> {
    let move_information = Move::get_move(move_id)?;

    let context = GetMoveContext {
        parent_move: move_information.0,
        move_list: move_information.1,
    };
    Ok(Template::render("non_root_move", &context))
}

#[get("/create/<move_id>")]
fn create_move(move_id: i32) -> Result<Template, Failure> {
    match Move::request_single_move(move_id)? {
        None => Err(Failure(Status::NoContent)),
        Some(parent_move) => {
            let mut context = HashMap::new();
            context.insert("parent_move", parent_move);
            Ok(Template::render("create", &context))
        }
    }
}

#[get("/create")]
fn create_root_move() -> Template {
    Template::render("create", &HashMap::<u32, u32>::new())
}

#[get("/delete/<move_id>")]
fn delete_move(move_id: i32) -> Result<Redirect, Failure> {
    match Move::request_single_move(move_id)? {
        None => Err(Failure(Status::NoContent)),
        Some(move_) => {
            Move::delete_move(move_id)?;
            match move_.parent {
                None => Ok(Redirect::to("/")),
                Some(parent_id) => {
                    let opt_parent = Move::request_single_move(parent_id)?;
                    match opt_parent {
                        None => Err(Failure(Status::InternalServerError)),
                        Some(parent) => Ok(Redirect::to(format!("/{}", parent.id).as_str())),
                    }
                }
            }
        }
    }
}

#[get("/<file..>")]
fn files(file: ::std::path::PathBuf) -> Option<NamedFile> {
    NamedFile::open(::std::path::Path::new("static/").join(file)).ok()
}
