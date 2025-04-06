// @generated automatically by Diesel CLI.

diesel::table! {
    patient_information (id) {
        id -> Int4,
        user_id -> Int4,
        date_of_birth -> Nullable<Date>,
        #[max_length = 10]
        gender -> Nullable<Varchar>,
        height -> Nullable<Float8>,
        weight -> Nullable<Float8>,
        #[max_length = 50]
        preferred_contact_method -> Nullable<Varchar>,
        #[max_length = 50]
        preferred_appointment_type -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    providers_information (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        license_number -> Varchar,
        #[max_length = 20]
        npi -> Varchar,
        #[max_length = 255]
        specialty -> Varchar,
        years_in_practice -> Int4,
        board_certified -> Bool,
        accepting_new_patients -> Bool,
        license_documents -> Nullable<Text>,
        digital_signature -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    providers_services (id) {
        id -> Int4,
        provider_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        category -> Varchar,
        description -> Nullable<Text>,
        price -> Float8,
        duration -> Int4,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 50]
        is_approved -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        profile_picture -> Nullable<Varchar>,
        role -> Varchar,
        status -> Varchar,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        address_street -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 100]
        state -> Nullable<Varchar>,
        #[max_length = 20]
        zip_code -> Nullable<Varchar>,
    }
}

diesel::joinable!(patient_information -> users (user_id));
diesel::joinable!(providers_information -> users (user_id));
diesel::joinable!(providers_services -> users (provider_id));

diesel::allow_tables_to_appear_in_same_query!(
    patient_information,
    providers_information,
    providers_services,
    users,
);
