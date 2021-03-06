extern crate diesel;

use backend_in_rust::database::*;
use backend_in_rust::models::Post;
use diesel::prelude::*;
fn main() {
    use backend_in_rust::schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
