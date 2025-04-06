use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

use crate::{
    db::establish_connection,
    providers_information::model::{
        NewProviderInformation, ProviderInformation, UpdateProviderInformation,
    },
    schema::providers_information,
};

pub async fn create_provider(
    new_provider: web::Json<NewProviderInformation>,
) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    diesel::insert_into(providers_information::table)
        .values(&new_provider.into_inner())
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json("Provider created successfully"))
}

pub async fn get_providers() -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let results = providers_information::table
        .load::<ProviderInformation>(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(results))
}

pub async fn get_provider_by_id(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let provider_id = path.into_inner();
    let conn = &mut establish_connection();
    let provider = providers_information::table
        .find(provider_id)
        .first::<ProviderInformation>(conn)
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(provider))
}

pub async fn delete_provider(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let provider_id = path.into_inner();
    let conn = &mut establish_connection();
    diesel::delete(providers_information::table.find(provider_id))
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json("Provider deleted successfully"))
}

pub async fn update_provider(
    path: web::Path<i32>,
    updated_data: web::Json<UpdateProviderInformation>,
) -> Result<HttpResponse, Error> {
    let provider_id = path.into_inner();
    let conn = &mut establish_connection();

    diesel::update(providers_information::table.find(provider_id))
        .set(&updated_data.into_inner())
        .execute(conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json("Provider updated successfully"))
}
