use chrono::{NaiveDateTime};
use super::schema::{areas, photos, sites, submissions, users};

#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Site, foreign_key="site_id")]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub site_id: i32
}

#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Submission, foreign_key="submission_id")]
pub struct Photo {
    pub id: i32,
    pub file_name: String,
    pub submission_id: i32,
    pub path: String
}

#[derive(Serialize, Identifiable, Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub location: (f64, f64),
    pub details: String,
}

#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
#[belongs_to(Area, foreign_key="area_id")]
pub struct Submission {
    pub id: i32,
    pub user_id: i32,
    pub area_id: i32,
    pub submission_date: NaiveDateTime,
    pub status: String
}

#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String
}