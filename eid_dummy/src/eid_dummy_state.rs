use crate::eid_dummy_evolvement::EidDummyEvolvement;
use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};
use std::convert::From;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EidDummyState {
    pub(crate) members: Vec<Member>,
}

impl EidState<EidDummyEvolvement> for EidDummyState {
    fn apply_log(&mut self, evolvements: &[EidDummyEvolvement]) -> Result<(), EidError> {
        let evolvement = evolvements.last().unwrap();
        self.apply(evolvement)
    }
    fn apply(&mut self, evolvement: &EidDummyEvolvement) -> Result<(), EidError> {
        match &evolvement {
            EidDummyEvolvement::Update { members }
            | EidDummyEvolvement::Add { members }
            | EidDummyEvolvement::Remove { members } => {
                self.members = members.clone();
                Ok(())
            }
        }
    }
    fn verify_client(&self, _: &Member) -> Result<bool, EidError> {
        Ok(true)
    }
    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        Ok(self.members.clone())
    }
}

impl From<&EidDummyState> for EidDummyState {
    fn from(state: &EidDummyState) -> Self {
        state.clone()
    }
}
