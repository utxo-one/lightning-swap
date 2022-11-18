
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use crate::schema::swaps;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Swap {
    pub uuid: String,
    pub amount: i32,
    pub payment_request: String,
    pub txid: Option<String>,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = swaps)]
pub struct NewSwap<> {
    pub uuid: String,
    pub amount: i32,
    pub payment_request: String,
    pub txid: Option<String>,
    pub status: String,
}