use crate::model::types::{Guest, Invitation};
use leptos::*;
use prelude::ServerFnError;
use std::sync::Mutex;

#[server(GetGuests, "/api")]
pub async fn get_guests_api(cx: Scope) -> Result<Vec<Guest>, ServerFnError> {
    let state = use_context::<AppState>(cx).unwrap();
    let mut conn = establish_connection(&state.database_url);
    Ok(get_guests(&mut conn))
}

#[server(AddGuest, "/api")]
pub async fn add_guest_api(cx: Scope, guest: Guest) -> Result<(), ServerFnError> {
    let state = use_context::<AppState>(cx).unwrap();
    let mut conn = establish_connection(&state.database_url);
    add_guest(&mut conn, guest);
    Ok(())
}

#[server(GetInvitations, "/api")]
pub async fn get_invitations() -> Result<Vec<Invitation>, ServerFnError> {
    let invitations = INVITE_LIST.lock().unwrap();
    Ok(invitations.clone())
}

#[server(AddInvitation, "/api")]
pub async fn add_invitation(invite: Invitation) -> Result<(), ServerFnError> {
    let mut invitations = INVITE_LIST.lock().unwrap();
    invitations.push(invite);
    Ok(())
}

// Global state
lazy_static::lazy_static! {
    static ref GUEST_LIST: Mutex<Vec<Guest>> = Mutex::new(Vec::new());
    static ref INVITE_LIST: Mutex<Vec<Invitation>> = Mutex::new(Vec::new());
}
