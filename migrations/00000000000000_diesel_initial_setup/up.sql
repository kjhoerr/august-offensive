 -- up.sql: Initial database migration for AO

CREATE OR REPLACE FUNCTION public.GETDATE() RETURNS TIMESTAMPTZ
    STABLE LANGUAGE SQL AS 'SELECT NOW()';

 -- base table for users
CREATE TABLE users (
    userid SERIAL,
    email VARCHAR(40) UNIQUE NOT NULL,
    firstname VARCHAR(20) NOT NULL,
    lastname VARCHAR(20) NOT NULL,
    password VARCHAR(255) NOT NULL,    -- intend to use bcrypt hash
    joindate DATE DEFAULT GETDATE(),
    activated BIT(1) DEFAULT B'0',     -- see activation_keys table
    PRIMARY KEY (userid)
);

 -- when activation is completed, key gets deleted and activated bit for user gets flipped to 1.
CREATE TABLE activation_keys (
    code VARCHAR(128),
    userid SERIAL,
    PRIMARY KEY (code),
    CONSTRAINT activation_keys_userid
        FOREIGN KEY (userid) REFERENCES users(userid)
);

 -- base table for games
CREATE TABLE games (
    gameid SERIAL,
    title VARCHAR(140) NOT NULL,
    gametypeid SERIAL NOT NULL,       -- future proofing: suppose different game types
    players SMALLINT NOT NULL,         -- number of players in a game
    waitfor INTEGER NOT NULL,          -- time in seconds to wait for player to complete a turn
    lastturn DATE DEFAULT GETDATE(),
    gamestate INTEGER DEFAULT 1,
    PRIMARY KEY (gameid)
);

 -- associative table: many users, many games
CREATE TABLE allegiances (
    gameid SERIAL,
    userid SERIAL,
    allegiance VARCHAR(140),           -- important/unimportant role for user (depends on gametype)
    ordernum SMALLINT NOT NULL,        -- user # spot in game
    playing SMALLINT DEFAULT 1,        -- default is binary: 0 means player OOP
    PRIMARY KEY (gameid, userid),
    CONSTRAINT users_games_gameid
        FOREIGN KEY (gameid) REFERENCES games(gameid),
    CONSTRAINT users_games_userid
        FOREIGN KEY (userid) REFERENCES users(userid)
);

 -- Regions are composed of nationstates
CREATE TABLE regions (
    regionid SERIAL,
    name VARCHAR(64) NOT NULL,
    abbreviation CHAR(2),
    bonus INTEGER NOT NULL,
    PRIMARY KEY (regionid)
);

 -- base table for nationstates
CREATE TABLE nationstates (
    nationid SERIAL,
    regionid SERIAL,
    name VARCHAR(64) NOT NULL,
    abbreviation CHAR(4),
    PRIMARY KEY (nationid),
    CONSTRAINT nationstates_regionid
        FOREIGN KEY (regionid) REFERENCES regions(regionid)
);

 -- nationstates border nationstates
CREATE TABLE borders (
    nationid SERIAL,
    borderid SERIAL,
    PRIMARY KEY (nationid, borderid),
    CONSTRAINT borders_nationid
        FOREIGN KEY (nationid) REFERENCES nationstates(nationid),
    CONSTRAINT borders_borderid
        FOREIGN KEY (borderid) REFERENCES nationstates(nationid)
);

 -- associative table: many games, many nationstates. Users control them.
CREATE TABLE games_nationstates (
    gameid SERIAL,
    nationid SERIAL,
    userid SERIAL,                     -- owner of nationstate for particular game
    PRIMARY KEY (gameid, nationid),
    CONSTRAINT games_nationstates_gameid
        FOREIGN KEY (gameid) REFERENCES games(gameid),
    CONSTRAINT games_nationstates_nationid
        FOREIGN KEY (nationid) REFERENCES nationstates(nationid),
    CONSTRAINT games_nationstates_userid
        FOREIGN KEY (userid) REFERENCES users(userid)
);
