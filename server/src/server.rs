#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

const server: &str = config::server;

const body: &str = include_str!("../static/index.html");
const logo: &[u8] = include_bytes!("../../doc/logo.png");

extern crate iron;
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    eprintln!("server @ http://{server}");
    Iron::new(get_form).http(server).unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html;Charset=Utf8));
    response.set_mut(body);
    return Ok(response);
}
