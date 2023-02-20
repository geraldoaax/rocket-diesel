use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(BlogPost {
        id,
        title: "Some title".to_string(),
        body: "Some body".to_string(),
        published: true,
    })
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost {
        id: 1,
        title: "My first post".to_string(),
        body: "This is my first post".to_string(),
        published: true,
    })
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .mount("/", routes![index])
        .mount("/blog-posts", routes![get_random_blog_post, get_blog_post])
    // .mount("/get_post", routes![get_blog_post])
}
