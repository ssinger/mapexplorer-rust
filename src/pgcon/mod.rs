//#![feature(proc_macro_hygiene,decl_macro)]
//#[macro_use] extern crate rocket;
use rocket::Rocket;
use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct Page {
    title : String
}

#[get("/")]
pub fn main()-> Template {
    let page = Page{title: String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/main",page)
        
}
    
#[get("/about")]
pub fn about()->Template {
    let page = Page{title: String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/about",page) 
}

#[get("/interest")]
pub fn interest()->Template {
    let page = Page{title: String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/interest",page) 
}

pub fn add_routes(rocket: Rocket)->Rocket {
    rocket
        .mount("/pgconmap",
               routes![main,
                       about,
                       interest])
}
