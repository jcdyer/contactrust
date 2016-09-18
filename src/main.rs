extern crate iron;
extern crate postgres;

use iron::prelude::*;
use iron::status;
use iron::headers;

use data::get_contacts;
use serializers::Jsonable;  // Trait for to_json

mod data;
mod serializers;

fn main() {
    let _server = Iron::new(view).http("localhost:3000").unwrap();
    println!("On port 3000");
}

fn view(_: &mut Request) -> IronResult<Response> {
    

    let mut resp = Response::with((status::Ok, get_contacts().to_json()));
    resp.headers.set((headers::ContentType::json()));
    Ok(resp)
}
