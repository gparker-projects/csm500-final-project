CREATE ROLE CLINICAL_USER WITH LOGIN PASSWORD 'csm500_clinical';

CREATE USER sbob WITH PASSWORD 'csm500';

CREATE SCHEMA "csm500";

\c csm500;

GRANT CONNECT ON DATABASE "csm500" TO CLINICAL_USER;
GRANT USAGE ON SCHEMA "csm500" TO CLINICAL_USER;
GRANT SELECT ON ALL TABLES IN SCHEMA "csm500" TO CLINICAL_USER;

GRANT CONNECT ON DATABASE "csm500" TO sbob;
GRANT USAGE ON SCHEMA "csm500" TO sbob;
GRANT SELECT ON users TO sbob;

INSERT INTO users (username, email, created_at) VALUES ('Mister Crabs', 'mcrabs@google.ca', '2026-06-06 12:00:00');
INSERT INTO users (username, email, created_at) VALUES ('Spongebob Squarepants', 'spants@google.ca', '2026-06-06 12:00:00');

select * from users;