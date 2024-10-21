use std::sync::Arc;

use crate::{
    domain::{
        EAlphabet, GetAllAlphabetsQuery, GetAlphabetByIdQuery, GetAlphabetByUserIdQuery, RAlphabet,
    },
    repository::{AlphabetRefRepository, AlphabetRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct AlphabetQueryService {
    alphabet_repository: Arc<AlphabetRepository>,
    alphabet_ref_repository: Arc<AlphabetRefRepository>,
}

impl AlphabetQueryService {
    pub fn new(
        alphabet_repository: Arc<AlphabetRepository>,
        alphabet_ref_repository: Arc<AlphabetRefRepository>,
    ) -> Self {
        Self {
            alphabet_repository,
            alphabet_ref_repository,
        }
    }
}

impl ServiceHandlerTrait<GetAlphabetByUserIdQuery, Vec<EAlphabet>> for AlphabetQueryService {
    async fn _handle(&self, query: GetAlphabetByUserIdQuery) -> Result<Vec<EAlphabet>, String> {
        Ok(self.alphabet_repository.find_by_user_id(&query.id).await)
    }
}

impl ServiceHandlerTrait<GetAllAlphabetsQuery, Vec<RAlphabet>> for AlphabetQueryService {
    async fn _handle(&self, query: GetAllAlphabetsQuery) -> Result<Vec<RAlphabet>, String> {
        Ok(self
            .alphabet_ref_repository
            .find_user_id_refs(&query.user_id)
            .await)
    }
}

impl ServiceHandlerTrait<GetAlphabetByIdQuery, RAlphabet> for AlphabetQueryService {
    async fn _handle(&self, query: GetAlphabetByIdQuery) -> Result<RAlphabet, String> {
        match self.alphabet_ref_repository.get_ref(&query.id).await {
            Some(alphabet) => Ok(alphabet),
            None => Err("alphabet not found".to_owned()),
        }
    }
}
