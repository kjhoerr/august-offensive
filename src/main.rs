// specify recurse depth for error_chain
#![recursion_limit = "1024"]

extern crate dotenv;
extern crate failure;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod schema;
pub mod routes;

use actix_web::{middleware, server, App};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use failure::Error;
use routes::*;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
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
            .prefix("/api")
            .handler("/callback", callback)
            .default_resource(|r| r.f(not_understood))
    }).bind(&bind_address)
        .unwrap()
        .start();

    println!("Started http server: {}", bind_address);
    let _ = sys.run();
    Ok(())
}
