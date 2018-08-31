table! {
    activation_keys (code) {
        code -> Varchar,
        userid -> Int4,
    }
}

table! {
    allegiances (gameid, userid) {
        gameid -> Int4,
        userid -> Int4,
        allegiance -> Nullable<Varchar>,
        ordernum -> Int2,
        playing -> Nullable<Int2>,
    }
}

table! {
    borders (nationid, borderid) {
        nationid -> Int4,
        borderid -> Int4,
    }
}

table! {
    games (gameid) {
        gameid -> Int4,
        title -> Varchar,
        gametypeid -> Int4,
        players -> Int2,
        waitfor -> Int4,
        lastturn -> Nullable<Date>,
        gamestate -> Nullable<Int4>,
    }
}

table! {
    games_nationstates (gameid, nationid) {
        gameid -> Int4,
        nationid -> Int4,
        userid -> Int4,
    }
}

table! {
    nationstates (nationid) {
        nationid -> Int4,
        regionid -> Int4,
        name -> Varchar,
        abbreviation -> Nullable<Bpchar>,
    }
}

table! {
    regions (regionid) {
        regionid -> Int4,
        name -> Varchar,
        abbreviation -> Nullable<Bpchar>,
        bonus -> Int4,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        email -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        password -> Varchar,
        joindate -> Nullable<Date>,
        activated -> Nullable<Bit>,
    }
}

joinable!(activation_keys -> users (userid));
joinable!(allegiances -> games (gameid));
joinable!(allegiances -> users (userid));
joinable!(games_nationstates -> games (gameid));
joinable!(games_nationstates -> nationstates (nationid));
joinable!(games_nationstates -> users (userid));
joinable!(nationstates -> regions (regionid));

allow_tables_to_appear_in_same_query!(
    activation_keys,
    allegiances,
    borders,
    games,
    games_nationstates,
    nationstates,
    regions,
    users,
);
