use crate::schema::providers_services;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ProviderService {
    pub id: i32,
    pub provider_id: i32,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub price: f64,
    pub duration: i32,
    pub status: String,
    pub is_approved: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = providers_services)]
pub struct NewProviderService {
    pub provider_id: i32,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub price: f64,
    pub duration: i32,
    pub status: String,
    pub is_approved: String,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = providers_services)]
pub struct UpdateProviderService {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub duration: Option<i32>,
    pub status: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = providers_services)]
pub struct UpdateProviderServiceApproval {
    pub is_approved: Option<String>,
}
