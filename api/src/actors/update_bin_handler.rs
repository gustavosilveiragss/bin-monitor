use crate::db_models::Bin;
use crate::messages::update_bin::UpdateBin;
use crate::schema::bin;
use crate::utils::DbActor;

use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<UpdateBin> for DbActor {
    type Result = QueryResult<Bin>;

    fn handle(&mut self, msg: UpdateBin, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Update Bin: Unable to establish connection");

        let updated_bin = diesel::update(bin::table.filter(bin::id.eq(msg.id)))
            .set::<UpdateBin>(msg)
            .get_result(&mut conn)?;

        Ok(updated_bin)
    }
}
