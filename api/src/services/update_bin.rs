use crate::{messages::update_bin::UpdateBin, AppState, DbActor};
use actix::Addr;
use actix_web::{
    patch,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[patch("/bin")]
pub async fn update_bin(state: Data<AppState>, body: Json<UpdateBin>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(UpdateBin {
            id: body.id.clone(),
            title: body.title.clone(),
            latitude: body.latitude.clone(),
            longitude: body.longitude.clone(),
            status: body.status.clone(),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("Bin not found"),
        _ => HttpResponse::InternalServerError().json("Failed to update bin"),
    }
}
