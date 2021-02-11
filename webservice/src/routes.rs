use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::*;
use crate::schema::*;
use crate::beans;
use crate::DbConn;

#[get("/")]
pub fn home() -> &'static str {
    "The API is up and running!"
}

#[get("/users")]
pub fn list_users(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::users::dsl::*;

    users.load(&conn.0).map_err(|err| -> String {
        println!("Error querying users: {:?}", err);
        "Error querying users from the database".into()
    }).map(Json)
}

/*
Lists the new submissions at their photos
*/
#[get("/new_submissions")]
pub fn list_new_submissions(conn: DbConn) -> Json<Vec<beans::SubmissionBean>> {

    // Retrieves the "NEW" submissions
    let submissions_list = submissions::table.filter(submissions::status.eq("NEW")).load::<Submission>(&conn.0).expect("Couldn't load submissions");

    // Retrieves the photos linked to the submissions
    let photos_list = Photo::belonging_to(&submissions_list).load::<Photo>(&conn.0).expect("Couldn't load messages");

    // Groups the photos by submissions
    let photos_list_grouped = photos_list.grouped_by(&submissions_list);

    // Isolates the id and the submission date of the submissions
    let submissions_bean_data_list= submissions_list.into_iter().map(|submission| {(submission.id, submission.submission_date)});

    // Isolates the path of the photos
    let photos_path_list_grouped: Vec<Vec<String>> = photos_list_grouped.into_iter().map(
        |photos: Vec<Photo>| photos.into_iter().map(|photo: Photo| photo.path).collect()
    ).collect();

    // Restrutures the data and returns it as JSON
    Json(submissions_bean_data_list.zip(photos_path_list_grouped).into_iter()
        .map(|((id, submission_date), photos)| {beans::SubmissionBean {id: id.to_string(), photos: photos, submission_date: submission_date}}).collect::<Vec<beans::SubmissionBean>>()
    )
    
}
