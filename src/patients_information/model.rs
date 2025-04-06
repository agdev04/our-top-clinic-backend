use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::patient_information;

#[derive(Queryable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = crate::schema::patient_information)]
pub struct PatientInformation {
    pub id: i32,
    pub user_id: i32,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub preferred_contact_method: Option<String>,
    pub preferred_appointment_type: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = patient_information)]
pub struct NewPatientInformation {
    pub user_id: i32,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub preferred_contact_method: Option<String>,
    pub preferred_appointment_type: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = patient_information)]
pub struct UpdatePatientInformation {
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub preferred_contact_method: Option<String>,
    pub preferred_appointment_type: Option<String>,
}
