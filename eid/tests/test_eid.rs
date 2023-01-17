#[macro_use]
extern crate lazy_static;

use std::fmt::Debug;

pub use rstest::*;
pub use rstest_reuse::{self, *};

use eid_dummy::eid_dummy_backend::EidDummyBackend;
pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_transcript::EidDummyTranscript;
use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_mls::eid_mls_transcript::EidMlsTranscript;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::member::Member;
use eid_traits::transcript::EidTranscript;
use eid_traits::types::EidError;

lazy_static! {
    static ref DUMMY_BACKEND: EidDummyBackend = EidDummyBackend::default();
    static ref MLS_BACKEND: EidMlsBackend = EidMlsBackend::default();
}

#[template]
#[rstest(client, backend,
case::EidDummy(& mut EidDummyClient::create_eid("test_key".as_bytes().to_vec(), & DUMMY_BACKEND).expect("creation failed"), & DUMMY_BACKEND),
case::EidMls(& mut EidMlsClient::create_eid(EidMlsClient::generate_pubkey(& MLS_BACKEND), & MLS_BACKEND).expect("creation failed"), & MLS_BACKEND),
)]
#[allow(non_snake_case)]
pub fn eid_clients<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
}

#[apply(eid_clients)]
fn add<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript =
        C::TranscriptProvider::new(client.export_transcript_state(), vec![], backend)
            .expect("Failed to create transcript");

    // Create Alice as a member with a random pk
    let alice = C::generate_initial_id(backend);
    let add_alice_evolvement = client.add(alice, backend).expect("failed to add member");

    // member list length unchanged before evolving
    let members = client.get_members().expect("failed to get members");
    assert_eq!(1, members.len());

    client
        .evolve(add_alice_evolvement.clone(), backend)
        .expect("Failed to apply state");

    let members = client.get_members().expect("failed to get members");
    assert!(members.contains(&alice));
    assert_eq!(2, members.len());

    transcript.add_evolvement(add_alice_evolvement.clone());
    assert_eq!(transcript.get_members(), members);

    // Try to add Alice a second time
    let member_in_eid_error = client
        .add(&alice, backend)
        .expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let bob = C::generate_initial_id(backend);
    let add_bob_evolvement = client.add(bob, backend).expect("failed to add member");
    client
        .evolve(add_bob_evolvement.clone(), backend)
        .expect("Failed to apply state");

    assert!(add_alice_evolvement.is_valid_successor(&add_bob_evolvement));
    transcript.add_evolvement(add_bob_evolvement.clone());

    let members = client.get_members().expect("failed to get members");
    assert_eq!(transcript.get_members(), members);
    assert!(members.contains(&bob));
    assert_eq!(3, members.len())
}

#[apply(eid_clients)]
fn remove<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript =
        C::TranscriptProvider::new(client.export_transcript_state(), vec![], backend)
            .expect("Failed to create transcript");

    let alice = C::generate_initial_id(backend);
    let evolvement_add = client.add(alice, backend).expect("failed to add member");
    client
        .evolve(evolvement_add.clone(), backend)
        .expect("Failed to apply state");

    transcript.add_evolvement(evolvement_add.clone());
    assert_eq!(
        transcript.get_members(),
        client.get_members().expect("failed to get members")
    );

    let evolvement_remove = client
        .remove(&alice, backend)
        .expect("failed to remove member");
    client
        .evolve(evolvement_remove.clone(), backend)
        .expect("Failed to apply remove on client state");

    assert!(evolvement_add.is_valid_successor(&evolvement_remove));
    transcript.add_evolvement(evolvement_remove.clone());
    assert_eq!(
        transcript.get_members(),
        client.get_members().expect("could not get members")
    );

    // Try to remove Alice a second time
    let member_not_in_eid_error = client
        .remove(&alice, backend)
        .expect_err("Removing non-existent member");
    assert!(matches!(
        member_not_in_eid_error,
        EidError::InvalidMemberError(..)
    ));
    let members = client.get_members().expect("failed to get members");
    assert!(!members.contains(&alice));
    assert_eq!(1, members.len());
}

#[apply(eid_clients)]
fn update<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript =
        C::TranscriptProvider::new(client.export_transcript_state(), vec![], backend)
            .expect("Failed to create transcript");
    let alice_before_update_1 = &client.get_members().expect("failed to get members")[0];

    let update_evolvement_1 = client.update(backend).expect("Updating client keys failed");
    client
        .evolve(update_evolvement_1.clone(), backend)
        .expect("Failed to apply update on client state");
    transcript.add_evolvement(update_evolvement_1.clone());
    assert_eq!(
        transcript.get_members(),
        client.get_members().expect("failed to get members")
    );

    let members_after_update_1 = client.get_members().expect("failed to get members");

    assert!(!members_after_update_1.contains(alice_before_update_1));
    assert_eq!(1, members_after_update_1.len());

    // Update Alice a second time
    let alice_before_update_2 = &members_after_update_1[0];
    let update_evolvement_2 = client.update(backend).expect("Updating client keys failed");
    client
        .evolve(update_evolvement_2.clone(), backend)
        .expect("Failed to apply update on client state");
    assert!(update_evolvement_1.is_valid_successor(&update_evolvement_2));
    transcript.add_evolvement(update_evolvement_2.clone());
    assert_eq!(
        transcript.get_members(),
        client.get_members().expect("failed to get members")
    );

    let members_after_update_2 = client.get_members().expect("failed to get members");

    assert!(!members_after_update_2.contains(alice_before_update_2));
    assert_eq!(1, members_after_update_2.len());
}
