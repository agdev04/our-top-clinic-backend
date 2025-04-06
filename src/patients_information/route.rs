use crate::patients_information::handler::{
    delete_patient_info, get_patient_info, update_patient_info,
};
use actix_web::web;

pub fn patient_information_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patient-information")
            .route("/{id}", web::get().to(get_patient_info))
            .route("/{id}", web::put().to(update_patient_info))
            .route("/{id}", web::delete().to(delete_patient_info)),
    );
}
