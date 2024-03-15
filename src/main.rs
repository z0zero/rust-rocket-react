#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::fs::FileServer;

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/assets", FileServer::from("templates/assets"))
}