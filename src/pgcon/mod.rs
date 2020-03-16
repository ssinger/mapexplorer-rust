use rocket::Rocket;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use postgres::{Client};
use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector,SslMethod,SslVerifyMode};

#[derive(serde::Serialize)]
struct Page {
    title : String
}

#[get("/")]
fn main()-> Template {
    let page = Page{title:
                    String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/main",page)
        
}
    
#[get("/about")]
fn about()->Template {
    let page = Page{title:
                    String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/about",page) 
}

#[get("/interest")]
fn interest()->Template {
    let page = Page{title:
                    String::from("MapExplorer - PGCon Map (Ottawa, Ontario)")};
    Template::render("pgcon/interest",page) 
}

#[derive(serde::Serialize)]
struct Point {
    lat: f64,
    lon: f64,
    description: String
}

#[get("/points")]
fn points()->Json<Vec<Point>>{
    let mut result = Vec::new();    .
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // Heroku uses a self signed cert.
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    
    let mut pgclient = Client::connect(&db_url,connector).unwrap();
    let query = "select lat,lon,point_type,description from points where point_type='pgcon'";
    let args=[];
    for row in pgclient.query(query,&args).unwrap() {
        result.push(Point{lat: row.get(0), lon:row.get(1), description: row.get(3)});        
    }
    
    
    Json(result)
}

pub fn add_routes(rocket: Rocket)->Rocket {
    rocket
        .mount("/pgconmap",
               routes![main,
                       about,
                       interest,
                       points])
}
