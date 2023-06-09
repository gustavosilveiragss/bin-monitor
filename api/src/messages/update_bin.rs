use crate::db_models::Bin;
use crate::schema::bin;
use actix::Message;
use diesel::{AsChangeset, QueryResult};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Message, Deserialize, AsChangeset)]
#[rtype(result = "QueryResult<Bin>")]
#[diesel(table_name = bin)]
pub struct UpdateBin {
    pub id: i32,
    pub title: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub status: Option<Decimal>,
}
