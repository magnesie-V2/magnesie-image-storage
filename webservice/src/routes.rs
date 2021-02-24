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

#[get("/")]
pub fn home() -> &'static str {
    "The API is up and running!"
}

/*
Lists the new submissions at their photos
*/
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
        .map(|submission| (submission.id, submission.submission_date));

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
            .map(|((id, submission_date), photos)| beans::SubmissionBean {
                id: id,
                photos: photos,
                submission_date: submission_date,
            })
            .collect::<Vec<beans::SubmissionBean>>(),
    )
}

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

#[post("/submit", data = "<data>")]
pub fn submit(conn: DbConn, content_type: &ContentType, data: Data) -> Status {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("user_id"),
        MultipartFormDataField::text("user_name"),
        MultipartFormDataField::text("site_id"),
        MultipartFormDataField::text("site_name"),
        MultipartFormDataField::text("site_details"),
        MultipartFormDataField::text("site_latitude"),
        MultipartFormDataField::text("site_longitude"),
        MultipartFormDataField::file("photos")
            .repetition(Repetition::infinite())
            .content_type(serde::__private::Some(mime::IMAGE_JPEG)),
    ]);

    let multipart_form_data_opt = MultipartFormData::parse(content_type, data, options);

    if multipart_form_data_opt.is_err() {
        match multipart_form_data_opt.err() {
            None => println!("Unknown error"),
            Some(err) => println!("Error in form data: {:?}", err),
        }
        return Status::BadRequest;
    }

    let mut multipart_form_data = multipart_form_data_opt.unwrap();

    let user_id_field_opt = multipart_form_data.texts.remove("user_id");
    let user_name_field_opt = multipart_form_data.texts.remove("user_name");
    let site_id_field_opt = multipart_form_data.texts.remove("site_id");
    let site_name_field_opt = multipart_form_data.texts.remove("site_name");
    let site_details_field_opt = multipart_form_data.texts.remove("site_details");
    let site_latitude_field_opt = multipart_form_data.texts.remove("site_latitude");
    let site_longitude_field_opt = multipart_form_data.texts.remove("site_longitude");
    let photos_field_opt = multipart_form_data.files.remove("photos");

    let mut user_id: i32 = 0;
    let mut user_name: String = "".to_string();
    let mut site_id: i32 = 0;
    let mut site_name: String = "".to_string();
    let mut site_details: String = "".to_string();
    let mut site_latitude: BigDecimal = BigDecimal::from_str("0.0").unwrap();
    let mut site_longitude: BigDecimal = BigDecimal::from_str("0.0").unwrap();
    let mut photos: Vec<FileField> = Vec::new();

    if let Some(mut user_id_vec) = user_id_field_opt {
        let user_id_field = user_id_vec.remove(0);
        user_id = user_id_field.text.parse::<i32>().unwrap();
    }

    if let Some(mut user_name_vec) = user_name_field_opt {
        let user_name_field = user_name_vec.remove(0);
        user_name = user_name_field.text;
    }

    if let Some(mut site_id_vec) = site_id_field_opt {
        let site_id_field = site_id_vec.remove(0);
        site_id = site_id_field.text.parse::<i32>().unwrap();
    }

    if let Some(mut site_name_vec) = site_name_field_opt {
        let site_name_field = site_name_vec.remove(0);
        site_name = site_name_field.text;
    }

    if let Some(mut site_details_vec) = site_details_field_opt {
        let site_details_field = site_details_vec.remove(0);
        site_details = site_details_field.text;
    }

    if let Some(mut site_latitude_vec) = site_latitude_field_opt {
        let site_latitude_field = site_latitude_vec.remove(0);
        site_latitude = BigDecimal::from_str(&site_latitude_field.text).unwrap();
    }

    if let Some(mut site_longitude_vec) = site_longitude_field_opt {
        let site_longitude_field = site_longitude_vec.remove(0);
        site_longitude = BigDecimal::from_str(&site_longitude_field.text).unwrap();
    }

    if let Some(photos_vec) = photos_field_opt {
        if photos_vec.len() < 2 {
            return Status::BadRequest;
        }
        photos = photos_vec;
    }

    if user_id == 0 {
        if user_name.len() == 0 {
            println!("User's name is too short.");
            return Status::BadRequest;
        }
        let inserted_user: InsertableUser = InsertableUser { name: user_name };
        let inserted_user_count = diesel::insert_into(users::table)
            .values(inserted_user)
            .execute(&conn.0);
        if inserted_user_count.is_err() {
            println!("Error inserting user: {:?}", inserted_user_count.err());
            return Status::InternalServerError;
        }
        if inserted_user_count.unwrap() > 0 {
            let inserted_user_id_option: Result<Option<i32>, Error> = users::table
                .select(max(users::id))
                .first::<Option<i32>>(&conn.0);
            if inserted_user_id_option.is_err() {
                println!(
                    "Error getting inserted user id: {:?}",
                    inserted_user_id_option.err()
                );
                return Status::InternalServerError;
            }
            match inserted_user_id_option.unwrap() {
                None => {
                    println!("Error getting inserted user id");
                    return Status::InternalServerError;
                }
                Some(inserted_user_id) => user_id = inserted_user_id,
            };
        }
    }

    if site_id == 0 {
        if site_name.len() == 0 {
            println!("Site's name is too short.");
            return Status::BadRequest;
        }
        let inserted_site: InsertableSite = InsertableSite {
            name: site_name,
            details: site_details,
            latitude: site_latitude,
            longitude: site_longitude,
        };
        let inserted_site_count = diesel::insert_into(sites::table)
            .values(inserted_site)
            .execute(&conn.0);
        if inserted_site_count.is_err() {
            println!("Error inserting site: {:?}", inserted_site_count.err());
            return Status::InternalServerError;
        }
        if inserted_site_count.unwrap() > 0 {
            let inserted_site_id_option: Result<Option<i32>, Error> = sites::table
                .select(max(sites::id))
                .first::<Option<i32>>(&conn.0);
            if inserted_site_id_option.is_err() {
                println!(
                    "Error getting inserted site id: {:?}",
                    inserted_site_id_option.err()
                );
                return Status::InternalServerError;
            }
            match inserted_site_id_option.unwrap() {
                None => {
                    println!("Error getting inserted site id");
                    return Status::InternalServerError;
                }
                Some(inserted_site_id) => site_id = inserted_site_id,
            };
        }
    }

    let now = Utc::now().naive_utc();

    let mut submission_id = 0; // TODO
    let inserted_submission: InsertableSubmission = InsertableSubmission {
        user_id: user_id,
        site_id: site_id,
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
        return Status::InternalServerError;
    }
    if inserted_submission_count.unwrap() > 0 {
        let inserted_submission_id_option: Result<Option<i32>, Error> = submissions::table
            .select(max(submissions::id))
            .first::<Option<i32>>(&conn.0);
        if inserted_submission_id_option.is_err() {
            println!(
                "Error getting inserted submission id: {:?}",
                inserted_submission_id_option.err()
            );
            return Status::InternalServerError;
        }
        match inserted_submission_id_option.unwrap() {
            None => {
                println!("Error getting inserted submission id");
                return Status::InternalServerError;
            }
            Some(inserted_submission_id) => submission_id = inserted_submission_id,
        };
    }

    let photos_folder_path = Path::new(&env::var("HOSTED_FILES_FOLDER").unwrap())
        .join(format!("{:0>4}", now.year()))
        .join(format!("{:0>2}", now.month()))
        .join(format!("{:0>2}", now.day()))
        .join(submission_id.to_string());

    let folder_creation_res = fs::create_dir_all(&photos_folder_path);
    if (&folder_creation_res).is_err() {
        println!(
            "Error creating folder `{:?}`: {:?}",
            photos_folder_path,
            folder_creation_res.err()
        );
        update_submission_status(submission_id, &conn, "ERROR");
        return Status::InternalServerError;
    }

    for (index, photo) in photos.iter().enumerate() {
        let new_file_name = format!("{}{}", index, ".jpg");
        let new_file_path = (&photos_folder_path).join(&new_file_name);
        let file_copy_res = fs::copy(&photo.path, &new_file_path);
        if file_copy_res.is_err() {
            println!(
                "Error copying file `{:?}` to `{:?}`: {:?}",
                &photo.path, &new_file_path, file_copy_res.err()
            );
            update_submission_status(submission_id, &conn, "ERROR");
            return Status::InternalServerError;
        }
        let inserted_photo: InsertablePhoto = InsertablePhoto {
            file_name: new_file_name,
            submission_id: submission_id,
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
            return Status::InternalServerError;
        }
    }

    update_submission_status(submission_id, &conn, "NEW");
    Status::Ok
}

fn update_submission_status(submission_id: i32, conn: &DbConn, status: &str) {
    let _update_submission_to_err_res =
        diesel::update(submissions::table.filter(submissions::id.eq(submission_id)))
            .set(submissions::status.eq(status))
            .execute(&conn.0);
}

#[get("/users")]
pub fn list_users(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::users::dsl::*;

    users
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying users: {:?}", err);
            "Error querying users from the database".into()
        })
        .map(Json)
}

#[get("/sites")]
pub fn list_sites(conn: DbConn) -> Result<Json<Vec<Site>>, String> {
    use crate::schema::sites::dsl::*;

    sites
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying sites: {:?}", err);
            "Error querying sites from the database".into()
        })
        .map(Json)
}
