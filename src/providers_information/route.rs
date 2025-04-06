use crate::providers_information::handler::{delete_provider, get_provider_by_id, update_provider};
use actix_web::web;

pub fn provider_information_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/providers-information")
            .route("/{id}", web::get().to(get_provider_by_id))
            .route("/{id}", web::put().to(update_provider))
            .route("/{id}", web::delete().to(delete_provider)),
    );
}
