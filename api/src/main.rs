use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

mod actors;
mod db_models;
mod messages;
mod schema;
mod services;
mod utils;

use services::*;
use utils::{get_pool, AppState, DbActor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("CONSOLE_URL")
        //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        //     .max_age(800);

        let cors = Cors::permissive(); // Currently allowing any origin

        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .wrap(cors)
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(fetch_bins::fetch_bins)
            .service(fetch_single_bin::fetch_single_bin)
            .service(update_bin::update_bin)
    })
    .bind(("0.0.0.0", 8080))
    //.bind(("127.0.0.1", 8080))
    .expect("Unable to bind http server")
    .run()
    .await
}
