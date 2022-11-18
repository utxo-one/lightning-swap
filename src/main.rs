use lightning_swap::controllers::swap_controller;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/swap",
        routes![
            swap_controller::index,
            swap_controller::store,
            swap_controller::show,
            swap_controller::delete
        ],
    )
}
