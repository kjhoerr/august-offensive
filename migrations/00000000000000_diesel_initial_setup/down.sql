-- down.sql: Destroy initial database migration for AO
DROP FUNCTION IF EXISTS public.GETDATE();

DROP TABLE users;
DROP TABLE activation_keys;
DROP TABLE games;
DROP TABLE allegiances;
DROP TABLE regions;
DROP TABLE nationstates;
DROP TABLE borders;
DROP TABLE games_nationstates;