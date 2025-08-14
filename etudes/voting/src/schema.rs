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

diesel::joinable!(votes -> options (option_id));
diesel::joinable!(votes -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(options, votes);
diesel::allow_tables_to_appear_in_same_query!(users, votes);
diesel::allow_tables_to_appear_in_same_query!(users, options);
