use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

use crate::{
    db::establish_connection,
    patients_information::model::{
        NewPatientInformation, PatientInformation, UpdatePatientInformation,
    },
    schema::patient_information,
};

pub async fn create_patient_info(
    new_info: web::Json<NewPatientInformation>,
) -> Result<HttpResponse> {
    let mut conn = establish_connection();

    diesel::insert_into(patient_information::table)
        .values(&new_info.into_inner())
        .execute(&mut conn)
        .expect("Error inserting");

    Ok(HttpResponse::Ok().json("Patient information created"))
}

pub async fn get_patient_infos() -> Result<HttpResponse> {
    let mut conn = establish_connection();
    let results = patient_information::table
        .load::<PatientInformation>(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({"status":"success", "data":results})))
}

pub async fn get_patient_info(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = establish_connection();
    let pid = id.into_inner();
    let info = patient_information::table
        .find(pid)
        .first::<PatientInformation>(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(info))
}

pub async fn update_patient_info(
    id: web::Path<i32>,
    updated_info: web::Json<UpdatePatientInformation>,
) -> Result<HttpResponse> {
    let mut conn = establish_connection();
    let pid = id.into_inner();

    diesel::update(patient_information::table.find(pid))
        .set(&updated_info.into_inner())
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json("Patient information updated"))
}

pub async fn delete_patient_info(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = establish_connection();
    let pid = id.into_inner();

    diesel::delete(patient_information::table.find(pid))
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json("Patient information deleted"))
}
