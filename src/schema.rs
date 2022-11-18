// @generated automatically by Diesel CLI.

diesel::table! {
    swaps (uuid) {
        uuid -> Text,
        amount -> Int4,
        payment_request -> Text,
        txid -> Nullable<Text>,
        status -> Text,
    }
}
