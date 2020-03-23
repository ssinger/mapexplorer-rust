use rocket::Rocket;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use postgres::{Client};
use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector,SslMethod,SslVerifyMode};


#[derive(serde::Serialize)]
struct Page {
    title: String,
    get_url: String
}

#[derive(serde::Serialize)]
struct Facility {
    lat: f64,
    lon: f64,
    name: String,
    city: String,
    url: String,
    washrooms: Option<String>,
    facility_type: String,
    source: String,
    description: String
}



#[get("/")]
fn index()->Template {
    let page = Page { title: String::from("Burlington & Oakville Recreational Spots"), get_url: String::from("") };
    
    Template::render("burlingtonrec/main",page)
}

#[get("/about")]
fn about()->Template {
     let page = Page { title: String::from("Burlington & Oakville Recreational Spots"), get_url: String::from("") };  
    Template::render("burlingtonrec/about",page)
}

#[get("/splash")]
fn splash()->Template {
    let page = Page { title: String::from("Burlington & Oakville Recreational Spots"),get_url: String::from("splash_list")};
    
    Template::render("burlingtonrec/interest",page)
}

#[get("/pools")]
fn pools()->Template {
 let page = Page { title: String::from("Burlington & Oakville Recreational Spots"),get_url: String::from("pool_list")};
    
    Template::render("burlingtonrec/interest",page)
}


fn query_facilities(facility_type: &str)->Json<Vec<Facility>> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // Heroku uses a self signed cert.
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let mut pgclient = Client::connect(&db_url,connector).unwrap();
    let mut result = Vec::new();    
    let query = "select lat,lon,name,city,url,washrooms,facility_type,source from facilities where facility_type like $1";
    for row in pgclient.query(query,&[&facility_type]).unwrap() {
        let opt_url: Option<&str> = row.get(4);
        
        let url:String = match opt_url {
            None=>String::from(""),
            Some(v)=>format!("<a href=\"{}\">Additional Information</a>",v)
        };

        
        let opt_name:Option<&str> =  row.get(2);
        let name = match opt_name {
            None=> String::from("Swimming Pool"),
            Some(v)=>String::from(v)
        };
        let source : String = row.get(7);
        let description = format!("{} {} <br>source:{}</br>",
                                  name,url,source);
        result.push(
            Facility{ lat: row.get(0),
                      lon: row.get(1),
                      name: name,
                      city: row.get(3),
                      url: url,
                      washrooms: row.get(5),
                      facility_type: row.get(6),
                      source: source,
                      description: description
            });
    }
    Json(result)
}

#[get("/pool_list")]
fn pool_list()->Json<Vec<Facility>> {
    query_facilities(&String::from("%Pool%"))
}

#[get("/splash_list")]
fn splash_list()->Json<Vec<Facility>> {
    query_facilities(&String::from("Spray Pad%"))
}




pub fn add_routes(rocket: Rocket)->Rocket {
    rocket.mount("/burlingtonrec",
                 routes![index,
                         about,
                         pools,
                         splash,
                         pool_list,
                         splash_list])
}
