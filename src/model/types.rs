use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{guests, invitations};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = guests)]
pub struct Guest {
    pub name: String,
    pub email: Option<String>,
    pub attending: bool,
    pub meal_preference: Option<String>, // Stored as CSV
    pub meal_extra_info: Option<String>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = invitations)]
pub struct Invitation {
    pub id: Option<i32>,
    pub lead_guest_name: String,
    pub sent: bool,
    pub acknowledged: bool,
    pub code: String,
}
