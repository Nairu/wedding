use crate::api::{add_guest, add_invitation, get_guests, get_invitations};
use crate::model::types::{Guest, Invitation};
use leptos::*;
use prelude::{signal, Read};
use server::Resource;
use std::collections::HashSet;

#[component]
pub fn AdminPanel() -> impl IntoView {
    // Local resource for guests
    let (guests, set_guests) = signal(Vec::<Guest>::new());
    let (invitations, set_invitations) = signal(Vec::<Invitation>::new());

    let async_guests = Resource::new(
        move || guests,
        |guests| load_data(guests)
    )

    // let guests = LocalResource::from_future(async { get_guests().await });

    // // Local resource for invitations
    // let invitations = LocalResource::from_future(async { get_invitations().await });

    // // Signals for adding new guests/invitations
    // let (new_guest_name, set_new_guest_name) = create_signal(String::new());
    // let (new_invitation_lead, set_new_invitation_lead) = create_signal(String::new());

    // // Action for adding a guest
    // let add_guest_action = create_action(move |guest_name: String| {
    //     let guest = Guest {
    //         name: guest_name.clone(),
    //         email: None,
    //         attending: false,
    //         meal_preference: HashSet::new(),
    //         meal_extra_info: None,
    //     };
    //     add_guest(guest)
    // });

    // // Action for adding an invitation
    // let add_invitation_action = create_action(move |lead_name: String| {
    //     let invitation = Invitation {
    //         lead_guest: Guest {
    //             name: lead_name.clone(),
    //             email: None,
    //             attending: false,
    //             meal_preference: HashSet::new(),
    //             meal_extra_info: None,
    //         },
    //         other_guests: vec![],
    //         sent: false,
    //         acknowledged: false,
    //         code: "example-code".to_string(),
    //     };
    //     add_invitation(invitation)
    // });

    // view! {
    //     <div class="admin-panel">
    //         <h1>"Admin Panel"</h1>

    //         // Guests Section
    //         <h2>"Guests"</h2>
    //         <Suspense fallback=move || view! { <p>"Loading guests..."</p> }>
    //             {move || guests.read().map(|guests| match guests {
    //                 Ok(guest_list) => view! {
    //                     <ul>
    //                         {guest_list.into_iter().map(|guest| view! {
    //                             <li>
    //                                 {guest.name} " - " {if guest.attending { "Attending" } else { "Not Attending" }}
    //                             </li>
    //                         }).collect::<Vec<_>>()}
    //                     </ul>
    //                 }.into_view(),
    //                 Err(_) => view! { <p>"Failed to load guests."</p> }.into_view(),
    //             })}
    //         </Suspense>
    //         <input
    //             type="text"
    //             placeholder="New guest name"
    //             prop:value=new_guest_name.get()
    //             on:input=move |ev| set_new_guest_name(event_target_value(&ev))
    //         />
    //         <button on:click=move |_| add_guest_action.dispatch(new_guest_name.get())>
    //             "Add Guest"
    //         </button>

    //         // Invitations Section
    //         <h2>"Invitations"</h2>
    //         <Suspense fallback=move || view! { <p>"Loading invitations..."</p> }>
    //             {move || invitations.read().map(|invitations| match invitations {
    //                 Ok(invite_list) => view! {
    //                     <ul>
    //                         {invite_list.into_iter().map(|invite| view! {
    //                             <li>
    //                                 {invite.lead_guest.name} " (Code: " {invite.code} ") - " {if invite.sent { "Sent" } else { "Not Sent" }}
    //                             </li>
    //                         }).collect::<Vec<_>>()}
    //                     </ul>
    //                 }.into_view(),
    //                 Err(_) => view! { <p>"Failed to load invitations."</p> }.into_view(),
    //             })}
    //         </Suspense>
    //         <input
    //             type="text"
    //             placeholder="New invitation lead guest"
    //             prop:value=new_invitation_lead.get()
    //             on:input=move |ev| set_new_invitation_lead(event_target_value(&ev))
    //         />
    //         <button on:click=move |_| add_invitation_action.dispatch(new_invitation_lead.get())>
    //             "Add Invitation"
    //         </button>
    //     </div>
    // }
}
