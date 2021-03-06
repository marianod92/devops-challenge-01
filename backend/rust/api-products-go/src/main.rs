
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello,{} year old named {}!", age,name)
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .launch();
}