pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid_traits::types::Member;
pub use rstest::*;
pub use rstest_reuse::{self, *};

#[template]
#[rstest(client, case::EIDDummy(&mut EidDummyClient::default()),)]
#[allow(non_snake_case)]
pub fn eid_clients(client: &mut impl EidClient) {}

#[apply(eid_clients)]
fn create<T: EidClient>(client: &mut T) {
    let keystore = T::KeyStoreProvider::default();
    let mut new_client = T::create_eid(keystore).expect("creation failed");
    let client_vector = new_client
        .state()
        .get_members()
        .expect("failed to get members");
    assert_eq!(client_vector.len(), 1);
    assert!(new_client.state().verify().unwrap());
}

#[apply(eid_clients)]
fn add(client: &mut impl EidClient) {
    let member = Member::default();
    let member_clone = member.clone();
    let evolvement = client.add(member).expect("failed to add member");
    client
        .state()
        .apply(evolvement)
        .expect("Failed to apply state");

    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(members.contains(&member_clone));
    assert_eq!(1, members.len())
}

#[apply(eid_clients)]
fn remove(client: &mut impl EidClient) {
    let member = Member::default();
    let member_to_remove = member.clone();
    let member_clone = member.clone();
    let evolvement_add = client.add(member).expect("failed to add member");
    client
        .state()
        .apply(evolvement_add)
        .expect("Failed to apply state");
    assert!(client.state().verify().unwrap());

    let evolvement_remove = client
        .remove(member_to_remove)
        .expect("failed to remove member");
    client
        .state()
        .apply(evolvement_remove)
        .expect("Failed to apply state");
    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(!members.contains(&member_clone));
    assert_eq!(0, members.len());
}

#[apply(eid_clients)]
fn update(client: &mut impl EidClient) {
    let member = Member::default();
    let member_clone = member.clone();
    let add_evolvement = client.add(member).expect("failed to add member");
    client
        .state()
        .apply(add_evolvement)
        .expect("Failed to apply state");

    let state = client.state();
    assert!(state.verify().unwrap());

    let update_evolvement = client.update().expect("Updating client keys failed");

    let members = state.get_members().expect("failed to get members");
}
