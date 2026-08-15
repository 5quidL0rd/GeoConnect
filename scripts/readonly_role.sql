-- readonly_role.sql
--
-- Creates the least-privilege role whose credential is compiled into shipped
-- GeoConnect binaries. Because that credential is extractable from any
-- distributed APK/MSI, this role must never be able to do anything beyond
-- reading the app's content tables.
--
-- Run as the admin (postgres) user. This file deliberately contains NO
-- password: set it separately with
--   ALTER ROLE geoconnect_readonly WITH PASSWORD '<generated>';
-- so no credential ever lands in the repository.

-- Role: login only, no elevated attributes of any kind.
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'geoconnect_readonly') THEN
        CREATE ROLE geoconnect_readonly
            LOGIN
            NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOREPLICATION NOBYPASSRLS;
    END IF;
END
$$;

-- Belt and braces: sessions for this role default to read-only transactions,
-- so even a grant mistake would still reject writes.
ALTER ROLE geoconnect_readonly SET default_transaction_read_only = on;

-- Access path: connect to the database, see the public schema.
GRANT CONNECT ON DATABASE railway TO geoconnect_readonly;
GRANT USAGE ON SCHEMA public TO geoconnect_readonly;

-- SELECT on exactly the tables the app reads, and nothing else.
-- (place_highlights is queried by src/place.rs and is required for the
-- place detail screens.)
GRANT SELECT ON
    regions,
    categories,
    facts,
    comparisons,
    places,
    place_highlights,
    travel_prep
TO geoconnect_readonly;

-- Everything else explicitly revoked. These are no-ops today (the role was
-- never granted them) but make the intended posture auditable.
REVOKE INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER
    ON ALL TABLES IN SCHEMA public FROM geoconnect_readonly;
REVOKE ALL ON ALL SEQUENCES IN SCHEMA public FROM geoconnect_readonly;
REVOKE CREATE ON SCHEMA public FROM geoconnect_readonly;
REVOKE CREATE, TEMPORARY ON DATABASE railway FROM geoconnect_readonly;

-- Future tables created by postgres must NOT be silently readable: reset any
-- default privileges that would auto-grant to this role. New tables therefore
-- require an explicit GRANT SELECT before the app can see them.
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA public
    REVOKE ALL ON TABLES FROM geoconnect_readonly;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA public
    REVOKE ALL ON SEQUENCES FROM geoconnect_readonly;
