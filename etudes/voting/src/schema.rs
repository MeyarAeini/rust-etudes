diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
options (id) {
    id -> Integer,
    name -> Text,
    description -> Text,
}
}

diesel::table! {
votes(user_id,option_id,ordinal){
    user_id -> Integer,
    option_id -> Integer,
    ordinal -> Integer,
}
}
