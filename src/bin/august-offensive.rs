extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate diesel;

use actix_web::{middleware, server, App, HttpRequest};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, world!"
}

// TODO describe change of direction in README
// TODO implement error-chain
// TODO match 0.1.0 functionality
fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS must be set");

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello-world");

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
}
