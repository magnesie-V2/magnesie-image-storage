use beans::*;
use diesel::{self, prelude::*};

use rocket::http::Status;
use rocket_contrib::json::Json;

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

#[put("/user", data = "<user>")]
pub fn create_user(conn: DbConn, user: Json<InsertableUser>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(users::table)
        .values(&user.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}


#[put("/submit", data = "<data>")]
pub fn submit(conn: DbConn, data: Json<SubmitedDataBean>) -> Result<String, String> {
    println!("user name: {}", &data.0.user.name);
    println!("site name: {}", &data.0.site.name);
    println!("site details: {}", &data.0.site.details);
    println!("site lat: {}", &data.0.site.latitude);
    println!("site long: {}", &data.0.site.longitude);
    
    Ok("merci".to_string())
}
