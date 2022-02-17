table! {
    use diesel::sql_types::*;
    photos (id) {
        id -> Integer,
        file_name -> Varchar,
        submission_id -> Integer,
        path -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    submissions (id) {
        id -> Integer,
        name -> Varchar,
        submission_date -> Timestamp,
        status -> Varchar,
    }
}

joinable!(photos -> submissions (submission_id));

allow_tables_to_appear_in_same_query!(
    photos,
    submissions,
);
