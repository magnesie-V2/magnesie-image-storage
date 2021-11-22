use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime};
use super::schema::{photos, submissions};

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

/// Submission struct (representation of the submissions table)
#[derive(Serialize, Identifiable, Queryable, Associations)]
pub struct Submission {
    pub id: i32,
    pub submission_date: NaiveDateTime,
    pub status: String
}

/// Insertable Submission struct (representation of the submissions table without the id that is inserted automatically)
#[derive(Insertable, Associations)]
#[table_name = "submissions"]
pub struct InsertableSubmission {
    pub submission_date: NaiveDateTime,
    pub status: String
}
