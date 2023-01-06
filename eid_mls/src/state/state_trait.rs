use eid_traits::state::EidState;
use eid_traits::types::EidError;
use openmls::prelude::ProcessedMessage;

use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;

pub trait EidMlsState: EidState<EidMlsEvolvement, EidMlsMember> + Clone + PartialEq {
    fn verify_client(&self, client: &EidMlsMember) -> Result<bool, EidError> {
        let members = self.get_members()?;
        Ok(members.contains(client))
    }

    fn apply_log(&mut self, log: &Vec<EidMlsEvolvement>) -> Result<(), EidError>
        where
            Self: Sized,
    {
        for evolvement in log.iter() {
            self.apply(evolvement)?;
        }
        Ok(())
    }

    fn apply_processed_message(&mut self, message: ProcessedMessage) -> Result<(), EidError> {
        todo!()
    }
}
