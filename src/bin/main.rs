#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use tokio::io;
use tokio::time::{sleep, Duration};

use rocket::request::Form;
use rocket::FromForm;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use serde::{Deserialize, Serialize};
#[derive(FromForm, Deserialize, Serialize)]
struct Task {
    complete: bool,
    description: String,
    custom: String,
}

#[post("/todo", data = "<task>")]
fn task(task: Option<Json<Task>>) -> Option<JsonValue> {
    // let todo2 = Task {
    //     complete: false,
    //     description: task.description.clone(),
    //     custom: "custom fernando".to_string(),
    // };
    // let todo1 = json!({
    //     "id":123456,
    //     "complete": false,
    //     "description": "",
    //     "custom":"BlaBlaBla Fernando"
    // });
    // todo1
    if let Some(task) = task {
        Some(json!({
            "id":123456,
            "complete": false,
            "description": task.description,
            "custom":"BlaBlaBla Fernando"
        }))
    } else {
        None
    }
}
#[derive(FromForm)]
struct Person {
    name: String,
    age: Option<u8>,
}

#[get("/hello?<person..>")]
fn hello(person: Option<Form<Person>>) -> String {
    if let Some(person) = person {
        if let Some(age) = person.age {
            format!("Hello, {} year old named {}!", age, person.name)
        } else {
            format!("Hello {}!", person.name)
        }
    } else {
        "We're gonna need a name, and only a name.".into()
    }
}

#[derive(FromForm)]
struct User {
    name: String,
}

#[get("/item?<id>&<user..>")]
fn item(id: usize, user: Form<User>) -> String {
    format!("Hello, {} year old named {}!", id, user.name)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/template?<name>")]
fn template(name: String) -> Template {
    let context: JsonValue = json!({
        "name":name,
    });
    Template::render("index", &context)
}

#[tokio::main]
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> io::Result<String> {
    sleep(Duration::from_secs(seconds)).await;
    Ok(format!("Waited for {} seconds", seconds))
}

#[tokio::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![delay, template, index, item, hello, task])
        .attach(Template::fairing())
        .launch();
}

// #[get("/user/<name>")]
// fn user(name: &RawStr) -> String {
//     format!("Hello, {}!", name.as_str())
// }

// #[get("/hello/<name>/<age>/<cool>", rank = 2)]
// fn hello(name: String, age: u8, cool: bool) -> String {
//     if cool {
//         format!("You're a cool {} anos, {}!", age, name)
//     } else {
//         format!("{}, we need to talk about your coolness.", name)
//     }
// }

// #[get("/hello?<name>", rank = 1)]
// fn wave(name: Option<String>) -> String {
//     name.map(|name| format!("Hi, {}!", name))
//         .unwrap_or_else(|| "Hello!".into())
// }
