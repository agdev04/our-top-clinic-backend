CREATE TABLE providers_information (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    license_number VARCHAR(255) NOT NULL,
    npi VARCHAR(20) NOT NULL,
    specialty VARCHAR(255) NOT NULL,
    years_in_practice INTEGER NOT NULL,
    board_certified BOOLEAN NOT NULL,
    accepting_new_patients BOOLEAN NOT NULL,
    license_documents TEXT,
    digital_signature TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);