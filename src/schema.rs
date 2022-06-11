table! {
    items (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
        list_id -> Int4,
    }
}

table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
    }
}

joinable!(items -> todos (list_id));

allow_tables_to_appear_in_same_query!(
    items,
    todos,
);
