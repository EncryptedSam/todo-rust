use crate::v1::controllers::user_controller;
use actix_web::web;

// Define your user routes here
pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/api/v1/user")
            .route("/sign-up", web::post().to(user_controller::sign_up))
            .route("/sign-in", web::post().to(user_controller::sign_in))
            .route(
                "/verify/{token}",
                web::get().to(user_controller::verify_token),
            ),
    );
}
