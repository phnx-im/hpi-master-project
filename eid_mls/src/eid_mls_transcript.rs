use eid_traits::state::EidState;
use eid_traits::transcript::EidTranscript;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::transcript_state::EidMlsTranscriptState;

pub struct EidMlsTranscript {
    trusted_state: EidMlsTranscriptState,
    current_state: EidMlsTranscriptState,
    log: Vec<EidMlsEvolvement>,
}

impl EidTranscript for EidMlsTranscript {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;
    type StateProvider = EidMlsTranscriptState;

    fn new(
        trusted_state: Self::StateProvider,
        log: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let mut transcript = EidMlsTranscript {
            trusted_state: trusted_state.clone_serde()?,
            current_state: trusted_state,
            log: vec![],
        };
        transcript.apply_log(log, backend)?;
        Ok(transcript)
    }

    fn evolve(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        self.current_state.apply(evolvement.clone(), backend)?;
        self.log.push(evolvement);
        Ok(())
    }

    fn log(&self) -> Vec<Self::EvolvementProvider> {
        self.log.clone()
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.current_state.get_members()
    }
}

impl EidMlsTranscript {
    fn apply_log(
        &mut self,
        mut log: Vec<EidMlsEvolvement>,
        backend: &EidMlsBackend,
    ) -> Result<(), EidError> {
        self.current_state.apply_log(log.clone(), backend)?;
        self.log.append(&mut log);
        Ok(())
    }
}
