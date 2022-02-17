use chrono::NaiveDateTime;
use std::io::Read;

use rocket::data::FromDataSimple;
use rocket::http::{ContentType, Status};
use rocket::{Outcome, Outcome::*};

/// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

/// Bean used by the GET `/new_submissions` route to retrieve data of the new submissions
#[derive(Serialize)]
pub struct SubmissionBean {
    pub id: i32,
    pub name: String,
    pub photos: Vec<String>,
    pub submission_date: NaiveDateTime,
}


/// Bean used by the POST `/change_submission_status` route to update the submission
#[derive(Deserialize)]
pub struct UpdateSubmissionBean {
    pub id: i32,
    pub status: String,
}

/// Implementation of the FromDataSimple trait for the UpdateSubmissionBean
impl FromDataSimple for UpdateSubmissionBean {
    type Error = String;

    fn from_data(
        request: &rocket::Request,
        data: rocket::Data,
    ) -> rocket::data::Outcome<Self, Self::Error> {
        // Ensure the content type is correct before opening the data.
        if request.content_type() != ContentType::parse_flexible("application/json").as_ref() {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        match serde_json::from_str(&string) {
            Ok(update_submission_bean) => Success(update_submission_bean),
            Err(_) => Failure((Status::UnprocessableEntity, string)),
        }
    }
}