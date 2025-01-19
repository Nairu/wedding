// @generated automatically by Diesel CLI.

diesel::table! {
    guests (name) {
        name -> Text,
        email -> Nullable<Text>,
        attending -> Bool,
        meal_preference -> Nullable<Text>,
        meal_extra_info -> Nullable<Text>,
    }
}

diesel::table! {
    invitations (id) {
        id -> Nullable<Integer>,
        lead_guest_name -> Text,
        sent -> Bool,
        acknowledged -> Bool,
        code -> Text,
    }
}

diesel::joinable!(invitations -> guests (lead_guest_name));

diesel::allow_tables_to_appear_in_same_query!(guests, invitations,);
