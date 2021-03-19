#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::templates::Template;

//módulos próprios
use backend_in_rust::routes::router_builder;
//importados

#[tokio::main]
async fn main() {
    rocket::ignite()
        .mount("/", router_builder())
        .attach(Template::fairing())
        .launch();
}
