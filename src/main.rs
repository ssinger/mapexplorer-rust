#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct Page {
    title : String
}


#[get("/")]
fn index()-> Template{
    let page = Page {title: String::from("MapExplorer - Exploring Communities Through Maps")};
    Template::render("home",page)
}

#[get("/")]
fn pgcon()-> Template {
    let page = Page{title: String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/main",page)
       
}

#[get("/")]
fn pgconabout()->Template {
    let page = Page{title: String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/about",page) 
}

#[get("/")]
fn pgconinterest()->Template {
    let page = Page{title: String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/interest",page) 
}

fn main() {
    rocket::ignite()
        .mount("/",routes![index])
        .mount("/pgconmap",routes![pgcon])
        .mount("/pgconmap/about",routes![pgconabout])
        .mount("/pgconmap/interest",routes![pgconinterest])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
    
