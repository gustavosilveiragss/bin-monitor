use crate::db_models::Bin;
use crate::messages::fetch_single_bin::FetchSingleBin;
use crate::schema::bin;
use crate::utils::DbActor;

use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchSingleBin> for DbActor {
    type Result = QueryResult<Bin>;

    fn handle(&mut self, msg: FetchSingleBin, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Single Bin: Unable to establish connection");

        let query = bin::table.find(msg.bin_id);

        let item = query.first::<Bin>(&mut conn)?;

        Ok(item)
    }
}
