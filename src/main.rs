#[macro_use]
extern crate rocket;
// #[macro_use]
// extern crate diesel;

// use self::diesel::prelude::*;
// use hello_rocket::models::Task;
// //use serde::de::{Deserialize};
// use serde::{Serialize};
use serde::{Deserialize};


#[derive(Deserialize)]
pub struct Task<'r> {
    pub description: &'r str,
    pub complete: &'r str
}



#[post("/post", data = "<task_json_string>")]
fn post(task_json_string: &str) {
    let t: Task = serde_json::from_str(task_json_string).unwrap();
    println!("Task Description: {}", t.description);
    println!("Task Complete: {}", t.complete);
}

// #[get("/tasks")]
// fn get_tasks() {
//     use hello_rocket::schema::tasks::dsl::*;

//     let connection = establish_connection();
//     let results = tasks
//         .filter(complete.eq("false"))
//         .limit(5)
//         .load::<Task>(&connection)
//         .expect("Error retrieving Tasks");

//     // Serialisation logic here
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![post])
}
