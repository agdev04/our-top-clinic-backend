CREATE TABLE patient_information (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    phone_number VARCHAR(20),
    date_of_birth DATE,
    gender VARCHAR(10),
    height FLOAT,
    weight FLOAT,
    address_street VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(100),
    zip_code VARCHAR(20),
    preferred_contact_method VARCHAR(50),
    preferred_appointment_type VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_patient_information_user_id ON patient_information(user_id);