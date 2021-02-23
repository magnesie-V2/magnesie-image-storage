use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime};
use super::schema::{photos, sites, submissions, users};


#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Submission, foreign_key="submission_id")]
pub struct Photo {
    pub id: i32,
    pub file_name: String,
    pub submission_id: i32,
    pub path: String
}

#[derive(Insertable, Associations)]
#[table_name = "photos"]
#[belongs_to(Submission, foreign_key="submission_id")]
pub struct InsertablePhoto {
    pub file_name: String,
    pub submission_id: i32,
    pub path: String
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub details: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal
}

#[derive(Insertable)]
#[table_name = "sites"]
pub struct InsertableSite {
    pub name: String,
    pub details: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal
}

#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
#[belongs_to(Site, foreign_key="site_id")]
pub struct Submission {
    pub id: i32,
    pub user_id: i32,
    pub site_id: i32,
    pub submission_date: NaiveDateTime,
    pub status: String
}

#[derive(Insertable, Associations)]
#[table_name = "submissions"]
#[belongs_to(User, foreign_key="user_id")]
#[belongs_to(Site, foreign_key="site_id")]
pub struct InsertableSubmission {
    pub user_id: i32,
    pub site_id: i32,
    pub submission_date: NaiveDateTime,
    pub status: String
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub name: String
}