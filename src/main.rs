#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};


// #[get("/hello/<name>/<place>")]
// fn hello(name: &str, age: u8) -> String {
//     format!("Hello, {} from {}!", name, place)
// }

#[derive(Deserialize, Serialize)]
struct Task<'r> {
    description: &'r str,
    complete: &'r str
}


#[post("/post", data = "<task_json_string>")]
fn post(task_json_string: &str) {
    let t: Task = serde_json::from_str(task_json_string).unwrap();
    println!("Task Description: {}", t.description);
    println!("Task Complete: {}", t.complete);
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![post])
}
