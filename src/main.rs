#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod messages;
pub mod routes;
#[macro_use]
pub mod schema;

use actix_rt;
use actix_web::{middleware, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::{env, io::Error};

#[cfg_attr(tarpaulin, skip)]
fn main() {
    if let Err(ref e) = run() {
        error!("error: {}", e);

        ::std::process::exit(1);
    }
}

// Run start-up for the server and dependencies
#[cfg_attr(tarpaulin, skip)]
fn run() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");

    env_logger::init();
    let sys = actix_rt::System::new("august-offensive");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::get_scope())
    })
    .bind(&bind_address)?
    .start();

    info!("Started http server: {}", bind_address);

    sys.run()?;
    Ok(())
}
