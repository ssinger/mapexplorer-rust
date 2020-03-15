#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::Rocket;
mod pgcon;

#[derive(serde::Serialize)]
struct Page {
    title : String
}


#[get("/")]
fn index()-> Template{
    let page = Page {title: String::from("MapExplorer - Exploring Communities Through Maps")};
    Template::render("home",page)
}



fn main() {
    let rocket = rocket::ignite()
        .mount("/",routes![index]);
    
    let rocket= pgcon::add_routes(rocket);
    
    rocket.mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
    
