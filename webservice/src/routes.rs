use std::{env, fs, path::Path, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{Datelike, Utc};
use diesel::{self, dsl::max, prelude::*, result::Error};

use rocket::http::Status;
use rocket_contrib::json::Json;

use rocket::http::ContentType;
use rocket::Data;

use rocket_multipart_form_data::{
    mime, FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
    Repetition,
};

use crate::beans;
use crate::models::*;
use crate::schema::*;
use crate::DbConn;

/// Home page
#[get("/")]
pub fn home() -> &'static str {
    "The API is up and running!"
}

/// Lists the new submissions at their photos
#[get("/new_submissions")]
pub fn list_new_submissions(conn: DbConn) -> Json<Vec<beans::SubmissionBean>> {
    // Retrieves the "NEW" submissions
    let submissions_list = submissions::table
        .filter(submissions::status.eq("NEW"))
        .load::<Submission>(&conn.0)
        .expect("Couldn't load submissions");

    // Retrieves the photos linked to the submissions
    let photos_list = Photo::belonging_to(&submissions_list)
        .load::<Photo>(&conn.0)
        .expect("Couldn't load messages");

    // Groups the photos by submissions
    let photos_list_grouped = photos_list.grouped_by(&submissions_list);

    // Isolates the id and the submission date of the submissions
    let submissions_bean_data_list = submissions_list
        .into_iter()
        .map(|submission| (submission.id, submission.name, submission.submission_date));

    // Isolates the path of the photos
    let photos_path_list_grouped: Vec<Vec<String>> = photos_list_grouped
        .into_iter()
        .map(|photos: Vec<Photo>| photos.into_iter().map(|photo: Photo| photo.path).collect())
        .collect();

    // Restrutures the data and returns it as JSON
    Json(
        submissions_bean_data_list
            .zip(photos_path_list_grouped)
            .into_iter()
            .map(|((id, name, submission_date), photos)| beans::SubmissionBean {
                id,
                name,
                photos,
                submission_date,
            })
            .collect::<Vec<beans::SubmissionBean>>(),
    )
}

/// Updates the status of the submission given as JSON to the given status 
#[post(
    "/change_submission_status",
    format = "application/json",
    data = "<submission>"
)]
pub fn update_submission(conn: DbConn, submission: beans::UpdateSubmissionBean) -> Status {
    let updated_row_count =
        diesel::update(submissions::table.filter(submissions::id.eq::<i32>(submission.id)))
            .set(submissions::status.eq(submission.status))
            .execute(&conn.0);

    match updated_row_count {
        Ok(0) => Status::BadRequest,
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

/// Saves the form data to the database and the photos to the filesystem
#[post("/submit", data = "<data>")]
pub fn submit(conn: DbConn, content_type: &ContentType, data: Data) -> Result<Json<i32>,Status> {
    // Form structure
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("name"),
        MultipartFormDataField::file("photos")
            .repetition(Repetition::infinite())
            .content_type(serde::__private::Some(mime::IMAGE_JPEG))
    ]);

    let multipart_form_data_opt = MultipartFormData::parse(content_type, data, options);

    if multipart_form_data_opt.is_err() {
        match multipart_form_data_opt.err() {
            None => println!("Unknown error"),
            Some(err) => println!("Error in form data: {:?}", err),
        }
        return Err(Status::BadRequest);
    }

    // the form data
    let mut multipart_form_data = multipart_form_data_opt.unwrap();

    let name_field_opt = multipart_form_data.texts.remove("name");
    let photos_field_opt = multipart_form_data.files.remove("photos");

    let mut name: String = "".to_string();

    if let Some(mut name_txt) = name_field_opt {
        name = name_txt.remove(0).text;
    }

    let mut photos: Vec<FileField> = Vec::new();

    // Extracts the form values from the form data
    // If there is less than 2 photos returns a 400 code
    if let Some(photos_vec) = photos_field_opt {
        if photos_vec.len() < 2 {
            return Err(Status::BadRequest);
        }
        photos = photos_vec;
    }

    let now = Utc::now().naive_utc();

    
    // Insert the submission and retrieve the id
    let mut submission_id = 0;
    // The submission is insterted with a "TEMP" status
    let inserted_submission: InsertableSubmission = InsertableSubmission {
        name,
        submission_date: now,
        status: "TEMP".to_string(),
    };
    let inserted_submission_count = diesel::insert_into(submissions::table)
        .values(inserted_submission)
        .execute(&conn.0);
    if inserted_submission_count.is_err() {
        println!(
            "Error inserting submission: {:?}",
            inserted_submission_count.err()
        );
        return Err(Status::InternalServerError);
    }
    if inserted_submission_count.unwrap() > 0 {
        // Retrieves the inserted submission id
        let inserted_submission_id_option: Result<Option<i32>, Error> = submissions::table
            .select(max(submissions::id))
            .first::<Option<i32>>(&conn.0);
        if inserted_submission_id_option.is_err() {
            println!(
                "Error getting inserted submission id: {:?}",
                inserted_submission_id_option.err()
            );
            return Err(Status::InternalServerError);
        }
        match inserted_submission_id_option.unwrap() {
            None => {
                println!("Error getting inserted submission id");
                return Err(Status::InternalServerError);
            }
            Some(inserted_submission_id) => submission_id = inserted_submission_id,
        };
    }

    // Storing folder path : ${HOSTED_FILES_FOLDER}/<YYYY>/<MM>/<DD>/<submission_id>
    let photos_folder_path = Path::new(&env::var("HOSTED_FILES_FOLDER").unwrap())
        .join(format!("{:0>4}", now.year()))
        .join(format!("{:0>2}", now.month()))
        .join(format!("{:0>2}", now.day()))
        .join(submission_id.to_string());

    // Creates the storing folder
    let folder_creation_res = fs::create_dir_all(&photos_folder_path);
    if (&folder_creation_res).is_err() {
        println!(
            "Error creating folder `{:?}`: {:?}",
            photos_folder_path,
            folder_creation_res.err()
        );
        update_submission_status(submission_id, &conn, "ERROR");
        return Err(Status::InternalServerError);
    }

    // For each photo
    for (index, photo) in photos.iter().enumerate() {
        // Save locally as : <photo_number>.jpg
        let new_file_name = format!("{}{}", index, ".jpg");
        let new_file_path = (&photos_folder_path).join(&new_file_name);
        let file_copy_res = fs::copy(&photo.path, &new_file_path);
        if file_copy_res.is_err() {
            println!(
                "Error copying file `{:?}` to `{:?}`: {:?}",
                &photo.path, &new_file_path, file_copy_res.err()
            );
            update_submission_status(submission_id, &conn, "ERROR");
            return Err(Status::InternalServerError);
        }
        // Inserts the photo into the database
        let inserted_photo: InsertablePhoto = InsertablePhoto {
            file_name: new_file_name,
            submission_id,
            path: new_file_path.into_os_string().into_string().unwrap()
        };
        let inserted_photo_count = diesel::insert_into(photos::table)
            .values(inserted_photo)
            .execute(&conn.0);
        if inserted_photo_count.is_err() {
            println!(
                "Error inserting photo: {:?}",
                inserted_photo_count.err()
            );
            update_submission_status(submission_id, &conn, "ERROR");
            return Err(Status::InternalServerError);
        }
    }

    // Updates the submission status to NEW
    update_submission_status(submission_id, &conn, "NEW");
    Ok(Json(submission_id))
}

/// Updates the status of the given submission to the given status 
fn update_submission_status(submission_id: i32, conn: &DbConn, status: &str) {
    let _update_submission_to_err_res =
        diesel::update(submissions::table.filter(submissions::id.eq(submission_id)))
            .set(submissions::status.eq(status))
            .execute(&conn.0);
}