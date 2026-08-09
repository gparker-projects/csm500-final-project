-- -------------------------------------------------------------------------------------
-- Loads seed database, schema and user data for the system.
-- -------------------------------------------------------------------------------------

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

