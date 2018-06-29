#[derive(Queryable, Serialize, Deserialize)]
pub struct Move {
    pub id: i32,
    pub parent: Option<i32>, 
    pub turn: i32,
    pub name_: String,
    pub special_name: Option<String>,
    pub line_description: Option<String>,
}