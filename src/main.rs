mod utils;
mod constants;
mod randomapi;

#[macro_use]
extern crate actix_web;
extern crate dotenv;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};
use actix_governor::{Governor, GovernorConfigBuilder};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    //Setup governor for rate limit
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            // Enable Governor middleware
            .wrap(Governor::new(&governor_conf))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(randomapi::random_proxy)

    })
        .bind("127.0.0.1:8998")?
        .run()
        .await
}
