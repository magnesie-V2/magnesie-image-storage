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

    sites (id) {
        id -> Integer,
        name -> Varchar,
        details -> Varchar,
        latitude -> Decimal,
        longitude -> Decimal,
    }
}

table! {
    use diesel::sql_types::*;

    submissions (id) {
        id -> Integer,
        user_id -> Integer,
        site_id -> Integer,
        submission_date -> Timestamp,
        status -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(photos -> submissions (submission_id));
joinable!(submissions -> sites (site_id));
joinable!(submissions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    photos,
    sites,
    submissions,
    users,
);
