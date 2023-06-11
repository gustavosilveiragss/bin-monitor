use crate::{messages::fetch_single_bin::FetchSingleBin, AppState, DbActor};
use actix::Addr;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

#[get("/bins/{id}")]
pub async fn fetch_single_bin(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let bin_id: i32 = path.into_inner() as i32;
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchSingleBin { bin_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No bin found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve bin"),
    }
}
