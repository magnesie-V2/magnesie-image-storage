table! {
    use diesel::sql_types::*;

    areas (id) {
        id -> Integer,
        name -> Varchar,
        site_id -> Integer,
    }
}

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
    use diesel_geometry::sql_types::*;

    sites (id) {
        id -> Integer,
        name -> Varchar,
        location -> Point,
        details -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

    submissions (id) {
        id -> Integer,
        user_id -> Integer,
        area_id -> Integer,
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

joinable!(areas -> sites (site_id));
joinable!(photos -> submissions (submission_id));
joinable!(submissions -> areas (area_id));
joinable!(submissions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    areas,
    photos,
    sites,
    submissions,
    users,
);
