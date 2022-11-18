use crate::models::swap_model::{NewSwap, Swap};
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn index_swap() -> Vec<Swap> {
    use crate::schema::swaps::dsl::*;
    let connection: &mut PgConnection = &mut crate::establish_connection();
    swaps.load::<Swap>(connection).expect("Error loading swaps")
}

pub fn store_swap(swap: NewSwap) -> Swap {
    use crate::schema::swaps;

    let connection: &mut PgConnection = &mut crate::establish_connection();

    let new_swap = NewSwap {
        uuid: swap.uuid,
        amount: swap.amount,
        payment_request: swap.payment_request,
        txid: swap.txid,
        status: swap.status,
    };

    diesel::insert_into(swaps::table)
        .values(&new_swap)
        .get_result(connection)
        .expect("Error saving new swap")
}

pub fn show_swap(swap_uuid: String) -> Swap {
    use crate::schema::swaps::dsl::*;

    let connection: &mut PgConnection = &mut crate::establish_connection();

    swaps
        .filter(uuid.eq(swap_uuid))
        .first::<Swap>(connection)
        .expect("Error loading swap")
}

pub fn delete_swap(swap_uuid: String) -> Swap {
    use crate::schema::swaps::dsl::*;

    let connection: &mut PgConnection = &mut crate::establish_connection();

    diesel::delete(swaps.filter(uuid.eq(swap_uuid)))
        .get_result(connection)
        .expect("Error deleting swap")
}
