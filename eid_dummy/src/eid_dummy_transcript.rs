use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::EidDummyMember;
use crate::eid_dummy_state::EidDummyState;
use eid_traits::transcript::EidTranscript;

#[derive(Default)]
pub struct EidDummyTranscript {
    trusted_state: EidDummyState,
    log: Vec<EidDummyEvolvement>,
}

impl EidTranscript for EidDummyTranscript {
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type StateProvider = EidDummyState;

    fn new(trusted_state: EidDummyState, log: Vec<EidDummyEvolvement>) -> Self {
        EidDummyTranscript { trusted_state, log }
    }

    fn add_evolvement(&mut self, evolvement: EidDummyEvolvement) {
        self.log.push(evolvement);
    }

    fn trusted_state(&self) -> EidDummyState {
        self.trusted_state.clone()
    }

    fn log(&self) -> Vec<EidDummyEvolvement> {
        self.log.clone()
    }
}
