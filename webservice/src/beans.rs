use chrono::{NaiveDateTime};

#[derive(Serialize)]
pub struct SubmissionBean {
    pub id: String,
    pub photos: Vec<String>,
    pub submission_date: NaiveDateTime,
}
