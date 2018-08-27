// specify recurse depth for error_chain
#![recursion_limit = "1024"]

extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate diesel;

pub mod errors;
pub mod schema;

use actix_web::{middleware, server, App, HttpRequest};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use errors::*;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS must be set");

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("august-offensive");

    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(index))
            .resource("/", |r| r.f(index))
    }).bind(&bind_address)
        .unwrap()
        .start();

    println!("Started http server: {}", bind_address);
    let _ = sys.run();
    Ok(())
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, world!"
}
