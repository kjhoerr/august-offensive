#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod messages;
pub mod routes;
#[macro_use]
pub mod schema;

use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2;
use std::{env, io::Error};

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() {
    if let Err(ref e) = run().await {
        error!("error: {}", e);

        ::std::process::exit(1);
    }
}

// Run start-up for the server and dependencies
#[cfg(not(tarpaulin_include))]
async fn run() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");

    env_logger::init();
    let pool = init_db_pool();

    info!("Started http server: {}", bind_address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(routes::get_scope())
    })
    .bind(&bind_address)?
    .run()
    .await
}

#[cfg(not(tarpaulin_include))]
fn init_db_pool() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL is not valid")
}
