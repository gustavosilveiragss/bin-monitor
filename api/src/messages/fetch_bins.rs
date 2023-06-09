use actix::Message;
use diesel::QueryResult;
use crate::db_models::Bin;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Bin>>")]
pub struct FetchBins;