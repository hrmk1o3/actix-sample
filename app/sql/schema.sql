DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
	id  BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (username)
);
GRANT CONNECT ON DATABASE testing_db TO test_user;
GRANT usage ON SCHEMA testing TO test_user;
GRANT SELECT,INSERT,UPDATE,DELETE on testing.users to test_user;
GRANT SELECT,UPDATE ON testing.users_id_seq TO test_user;
