#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{JSON, Value};
use rocket_contrib::Template;


#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct City {
    id: u32,
    sehir: String
}

#[get("/city/<id>")]
fn get(id: u32) -> JSON<City> {
    if id == 63 {
        JSON(City {
            id: 63,
            sehir: "Şanlıurfa".to_string()
        })
    } else {
        JSON(City {
            id: 0,
            sehir: "Girilen şehir id'si bulunamadı. Lütfen geçerli bir id girin.".to_string()
        })
    }
}

#[get("/")]
fn getall_city() -> String {
    "All cities..".to_string()
}

#[get("/index/<name>")]
fn index(name: String) -> Template {
    let context = TemplateContext {
        name: name,
        items: vec!["Learn", "Practice", "Rust"].iter().map(|s| s.to_string()).collect()
    };
    Template::render("index", &context)
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {

    rocket::ignite()
        .mount("/", routes![get, getall_city, index])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
