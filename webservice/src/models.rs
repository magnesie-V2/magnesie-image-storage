use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime};
use super::schema::{photos, sites, submissions, users};

/// Photo struct (representation of the photos table)
#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Submission, foreign_key="submission_id")]
pub struct Photo {
    pub id: i32,
    pub file_name: String,
    pub submission_id: i32,
    pub path: String
}

/// Insertable Photo struct (representation of the photos table without the id that is inserted automatically)
#[derive(Insertable, Associations)]
#[table_name = "photos"]
#[belongs_to(Submission, foreign_key="submission_id")]
pub struct InsertablePhoto {
    pub file_name: String,
    pub submission_id: i32,
    pub path: String
}

/// Site struct (representation of the sites table)
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub details: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal
}

/// Insertable Site struct (representation of the sites table without the id that is inserted automatically)
#[derive(Insertable)]
#[table_name = "sites"]
pub struct InsertableSite {
    pub name: String,
    pub details: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal
}

/// Submission struct (representation of the submissions table)
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

/// Insertable Submission struct (representation of the submissions table without the id that is inserted automatically)
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

/// User struct (representation of the users table)
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String
}

/// Insertable User struct (representation of the users table without the id that is inserted automatically)
#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub name: String
}