use models::Move;
use models::NewMove;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Failure, NamedFile, Redirect};
use rocket_contrib::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct GetMoveContext {
    parent_move: Option<Move>,
    current_move: Move,
    child_moves: Vec<Move>,
}

#[derive(FromForm)]
struct CreateMoveForm {
    pub parent_id: Option<i32>,
    pub name_: String,
    pub special_name: Option<String>,
    pub line_description: Option<String>,
}

#[get("/")]
fn index() -> Result<Template, Failure> {
    let root_moves = Move::get_root_moves()?;

    let mut context = HashMap::new();
    context.insert("child_moves", root_moves);
    Ok(Template::render("index", &context))
}

#[get("/<move_id>")]
fn get_move(move_id: i32) -> Result<Template, Failure> {
    let move_information = Move::get_move_with_parent(move_id)?;

    let context = GetMoveContext {
        parent_move: move_information.0,
        current_move: move_information.1,
        child_moves: move_information.2,
    };
    Ok(Template::render("move_details", &context))
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

#[post("/create", data = "<creation_form>")]
fn create_move_from_form(creation_form: Form<CreateMoveForm>) -> Result<Redirect, Failure> {
    let creation_form = creation_form.get();
    let move_ = NewMove {
        parent: creation_form.parent_id,
        name_: creation_form.name_.clone(),
        special_name: creation_form.special_name.clone(),
        line_description: creation_form.line_description.clone(),
    };
    let new_move_id = Move::create_move(move_)?;

    Ok(Redirect::to(format!("/{}", new_move_id).as_str()))
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
