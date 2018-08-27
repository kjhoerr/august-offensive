table! {
    users(userid) {
        userid -> Serial,
        email -> VarChar,
        firstname -> VarChar,
        lastname -> VarChar,
        password -> VarChar,
        joindate -> Date,
        activated -> Bool,
    }
}
table! {
    activation_keys(code) {
        code -> VarChar,
        userid -> Serial,
    }
}
table! {
    games(gameid) {
        gameid -> Serial,
        title -> VarChar,
        gametypeid -> Serial,
        players -> SmallInt,
        waitfor -> Integer,
        lastturn -> Date,
        gamestate -> Integer,
    }
}
table! {
    allegiances(gameid, userid) {
        gameid -> Serial,
        userid -> Serial,
        allegiance -> VarChar,
        ordernum -> SmallInt,
        playing -> SmallInt,
    }
}
table! {
    regions(regionid) {
        regionid -> Serial,
        name -> VarChar,
        abbreviation -> Char,
        bonus -> Integer,
    }
}
table! {
    nationstates(nationid) {
        nationid -> Serial,
        regionid -> Serial,
        name -> VarChar,
        abbreviation -> Char,
    }
}
table! {
    borders(nationid, borderid) {
        nationid -> Serial,
        borderid -> Serial,
    }
}
table! {
    regions_nationstates(gameid, nationid) {
        gameid -> Serial,
        nationid -> Serial,
        userid -> Serial,
    }
}