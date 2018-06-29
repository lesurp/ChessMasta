table! {
    moves (id) {
        id -> Integer,
        parent -> Nullable<Integer>,
        turn -> Integer,
        name_ -> Text,
        special_name -> Nullable<Text>,
        line_description -> Nullable<Text>,
    }
}
