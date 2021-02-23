use chrono::NaiveDateTime;
use std::io::Read;

use rocket::data::FromDataSimple;
use rocket::http::{ContentType, Status};
use rocket::{Outcome, Outcome::*};

use crate::models::{Site, User};

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

/*
Bean used by the GET `/new_submissions` route to retrieve data of the new submissions
*/
#[derive(Serialize)]
pub struct SubmissionBean {
    pub id: i32,
    pub photos: Vec<String>,
    pub submission_date: NaiveDateTime,
}

/*
Bean used by the POST `/change_submission_status` route to update the submission
*/
#[derive(Deserialize)]
pub struct UpdateSubmissionBean {
    pub id: i32,
    pub status: String,
}

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

#[derive(Deserialize)]
pub struct SubmitedDataBean {
    pub user: User,
    pub site: Site,
    pub photos: Vec<SubmitedPhotoBean>,
}

#[derive(Deserialize)]
pub struct SubmitedPhotoBean {
    // #[serde(with = "base64")]
    // pub base64: Vec<u8>,
    pub base64: String
}

// mod base64 {
//     extern crate base64;
//     use serde::{de, Deserialize, Deserializer};

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = <&str>::deserialize(deserializer)?;
//         // let content = s.split(',').collect::<Vec<&str>>();
//         // let mut str: &str = s;
//         // if content.len() == 2 {
//         //     str = content[1];
//         // }
//         base64::decode(s).map_err(de::Error::custom)
//     }
// }
