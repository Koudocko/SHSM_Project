use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres@localhost/SHSM_Project";

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
