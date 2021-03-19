//rocket
use rocket::request::Form;
use rocket::FromForm;
use rocket::{routes, Route};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;
//serde
use serde::{Deserialize, Serialize};
//tokio
use tokio::io;
use tokio::time::{sleep, Duration};

//módulos próprios
use crate::controllers::posts::create_post;
//fim módulos próprios

pub fn router_builder() -> Vec<Route> {
    let routes = routes![
        create_new_post,
        task,
        hello_person,
        user,
        index,
        async_delay
    ];

    #[derive(Deserialize, Serialize)]
    struct Post {
        title: String,
        body: String,
    }
    #[tokio::main]
    #[post("/post", data = "<post>")]
    async fn create_new_post(post: Json<Post>) -> JsonValue {
        let new_post = create_post(&post.title, &post.body).await;
        json!(&new_post)
    }
    // rotas exemplo
    #[derive(FromForm, Deserialize, Serialize)]
    struct Task {
        complete: bool,
        description: String,
        owner: String,
    }

    #[post("/todo", data = "<task>")]
    fn task(task: Option<Json<Task>>) -> Option<JsonValue> {
        if let Some(task) = task {
            let todo = Task {
                complete: task.complete,
                description: task.description.to_string(),
                owner: task.owner.to_string(),
            };
            Some(json!(todo))
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
    fn hello_person(person: Option<Form<Person>>) -> String {
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
    fn user(id: usize, user: Form<User>) -> String {
        format!("Hello, {} year old named {}!", id, user.name)
    }

    #[get("/")]
    fn index() -> Template {
        let context: JsonValue = json!({
            "name":"Fernando Marca Magalhães",
        });
        Template::render("index", &context)
    }

    #[tokio::main]
    #[get("/delay/<seconds>")]
    async fn async_delay(seconds: u64) -> io::Result<String> {
        sleep(Duration::from_secs(seconds)).await;
        Ok(format!("Waited for {} seconds", seconds))
    }
    //retorno de todas as rotas
    routes
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
