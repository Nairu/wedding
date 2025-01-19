use crate::model::types::{Guest, Invitation};
use crate::schema::{guests, invitations};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use leptos::prelude::ServerFnError;
use ssr::db;

#[cfg(feature = "ssr")]
pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use diesel::{sqlite::SqliteConnection, Connection};
    use leptos::server_fn::ServerFnError;

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url).map_err(|e| ServerFnError::new(e))
    }
}

// Get all guests
pub async fn get_guests() -> Result<Vec<Guest>, ServerFnError> {
    let mut conn = db().await?;
    Ok(guests::table
        .load::<Guest>(&mut conn)
        .expect("Error loading guests"))
}

// Add a new guest
pub fn add_guest(guest: Guest) {
    diesel::insert_into(guests::table)
        .values(&guest)
        .execute(conn)
        .expect("Error adding new guest");
}

// Get all invitations
pub fn get_invitations(conn: &mut SqliteConnection) -> Vec<Invitation> {
    invitations::table
        .load::<Invitation>(conn)
        .expect("Error loading invitations")
}

// Add a new invitation
pub fn add_invitation(conn: &mut SqliteConnection, invitation: Invitation) {
    diesel::insert_into(invitations::table)
        .values(&invitation)
        .execute(conn)
        .expect("Error adding new invitation");
}
