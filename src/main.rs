extern crate actix_rt;
extern crate actix_web;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod routes;
pub mod schema;

use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use routes::*;
use std::env;
use std::io::Error;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");

    env_logger::init();
    let sys = actix_rt::System::new("august-offensive");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api")
                .service(web::scope("/callback").default_service(web::route().to(callback)))
                .default_service(web::route().to(not_understood)),
        )
    })
    .bind(&bind_address)?
    .start();

    info!("Started http server: {}", bind_address);

    sys.run()
}
