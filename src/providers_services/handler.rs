use crate::db::establish_connection;
use crate::providers_services::model::{NewProviderService, ProviderService};
use crate::schema::providers_services;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

use super::model::{UpdateProviderService, UpdateProviderServiceApproval};

// Create a new provider service
pub async fn create_provider_service(
    new_service: web::Json<NewProviderService>,
) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    diesel::insert_into(providers_services::table)
        .values(&new_service.into_inner())
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json("Provider Service created"))
}

// Get all provider services
pub async fn get_all_provider_services() -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let services = providers_services::table
        .load::<ProviderService>(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(services))
}

// Get services by provider ID
pub async fn get_services_by_provider(provider_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let services = providers_services::table
        .filter(providers_services::provider_id.eq(provider_id.into_inner()))
        .load::<ProviderService>(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(services))
}

// Get single provider service by ID
pub async fn get_provider_service_by_id(service_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let service = providers_services::table
        .find(service_id.into_inner())
        .first::<ProviderService>(conn)
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(service))
}

// Update a provider service
pub async fn update_provider_service(
    service_id: web::Path<i32>,
    updated_data: web::Json<UpdateProviderService>,
) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    diesel::update(providers_services::table.find(service_id.into_inner()))
        .set(&updated_data.into_inner())
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json("Provider Service updated"))
}

// Update approval status only
pub async fn update_provider_service_approval(
    service_id: web::Path<i32>,
    approval_data: web::Json<UpdateProviderServiceApproval>,
) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    diesel::update(providers_services::table.find(service_id.into_inner()))
        .set(&approval_data.into_inner())
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json("Approval status updated"))
}

// Delete a provider service
pub async fn delete_provider_service(service_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    diesel::delete(providers_services::table.find(service_id.into_inner()))
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json("Provider Service deleted"))
}
