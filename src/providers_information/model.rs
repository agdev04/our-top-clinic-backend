use crate::schema::providers_information;
use diesel::{prelude::AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = providers_information)]
pub struct ProviderInformation {
    pub id: i32,
    pub user_id: i32,
    pub license_number: String,
    pub npi: String,
    pub specialty: String,
    pub years_in_practice: i32,
    pub board_certified: bool,
    pub accepting_new_patients: bool,
    pub license_documents: Option<String>,
    pub digital_signature: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "providers_information"]
pub struct NewProviderInformation {
    pub user_id: i32,
    pub license_number: String,
    pub npi: String,
    pub specialty: String,
    pub years_in_practice: i32,
    pub board_certified: bool,
    pub accepting_new_patients: bool,
    pub license_documents: Option<String>,
    pub digital_signature: Option<String>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "providers_information"]
pub struct UpdateProviderInformation {
    pub license_number: Option<String>,
    pub npi: Option<String>,
    pub specialty: Option<String>,
    pub years_in_practice: Option<i32>,
    pub board_certified: Option<bool>,
    pub accepting_new_patients: Option<bool>,
    pub license_documents: Option<String>,
    pub digital_signature: Option<String>,
}
