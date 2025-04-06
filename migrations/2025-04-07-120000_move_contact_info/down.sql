ALTER TABLE users
  DROP COLUMN phone_number,
  DROP COLUMN address_street,
  DROP COLUMN city,
  DROP COLUMN state,
  DROP COLUMN zip_code;

ALTER TABLE patient_information
  ADD COLUMN phone_number VARCHAR(20),
  ADD COLUMN address_street VARCHAR(255),
  ADD COLUMN city VARCHAR(100),
  ADD COLUMN state VARCHAR(100),
  ADD COLUMN zip_code VARCHAR(20);