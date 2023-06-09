use crate::db_models::Bin;
use crate::messages::fetch_bins::FetchBins;
use crate::schema::bin;
use crate::utils::DbActor;

use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchBins> for DbActor {
    type Result = QueryResult<Vec<Bin>>;

    fn handle(&mut self, _msg: FetchBins, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Bins: Unable to establish connection");

        let query = bin::table;

        let menu = query.load::<Bin>(&mut conn).unwrap();

        Ok(menu)
    }
}
