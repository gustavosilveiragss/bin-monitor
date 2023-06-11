use crate::db_models::Bin;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Bin>")]
pub struct FetchSingleBin {
    pub bin_id: i32,
}