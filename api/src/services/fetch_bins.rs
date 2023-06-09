use crate::{messages::fetch_bins::FetchBins, AppState, DbActor};
use actix::Addr;
use actix_web::{get, web::Data, HttpResponse, Responder};

#[get("/bins")]
pub async fn fetch_bins(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchBins).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No bins found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve bins"),
    }
}
