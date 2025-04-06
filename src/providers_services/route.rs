use crate::providers_services::handler::{
    create_provider_service, delete_provider_service, get_services_by_provider,
    update_provider_service, update_provider_service_approval,
};
use actix_web::web;

pub fn providers_services_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patient-information")
            .route("", web::post().to(create_provider_service))
            .route("/{id}", web::get().to(get_services_by_provider))
            .route(
                "/update-provider-service/{id}",
                web::put().to(update_provider_service),
            )
            .route(
                "/update-provider-service-approval/{id}",
                web::put().to(update_provider_service_approval),
            )
            .route("/{id}", web::delete().to(delete_provider_service)),
    );
}
