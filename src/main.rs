#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
mod pgcon;
mod penlake;
#[derive(serde::Serialize)]
struct Page {
    title : String
}


#[get("/")]
fn index()-> Template{
    let page = Page {title: String::from("MapExplorer - Exploring Communities Through Maps")};
    Template::render("home",page)
}

#[get("/about")]
fn about()->Template {
    let page = Page {title: String::from("MapExplorer - Exploring Communities Through Maps")};
    Template::render("about",page)
}


fn main() {
    let rocket = rocket::ignite()
        .mount("/",routes![index,about]);
    
    let rocket= pgcon::add_routes(rocket);
    let rocket = penlake::add_routes(rocket);
    rocket.mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
    
