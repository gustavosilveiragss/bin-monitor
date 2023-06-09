#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
#[diesel(table_name = bin)]
pub struct Bin {
    pub id: i32,
    pub title: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub status: Option<Decimal>,
}
