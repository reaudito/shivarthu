use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::{CallError, ErrorCode, ErrorObject},
};
use parity_scale_codec::Codec;
use project_tips_runtime_api::ProjectTipsApi as ProjectTipsRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

type ProjectId = u64;

#[rpc(client, server)]
pub trait ProjectTipsApi<BlockHash, AccountId> {
    #[method(name = "projecttips_evidenceperiodendblock")]
    fn get_evidence_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "projecttips_stakingperiodendblock")]
    fn get_staking_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "projecttips_drawingperiodend")]
    fn get_drawing_period_end(
        &self,
        project_id: ProjectId,
        at: Option<BlockHash>,
    ) -> RpcResult<(u64, u64, bool)>;
    #[method(name = "projecttips_commitendblock")]
    fn get_commit_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "projecttips_voteendblock")]
    fn get_vote_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "projecttips_selectedjuror")]
    fn selected_as_juror(
        &self,
        project_id: ProjectId,
        who: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<bool>;
}

/// A struct that implements the `SumStorageApi`.
pub struct ProjectTips<C, M> {
    // If you have more generics, no need to SumStorage<C, M, N, P, ...>
    // just use a tuple like SumStorage<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> ProjectTips<C, M> {
    /// Create new `SumStorage` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

/// Error type of this RPC api.
pub enum Error {
    /// The transaction was not decodable.
    DecodeError,
    /// The call to runtime failed.
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
            Error::DecodeError => 2,
        }
    }
}

impl<C, Block, AccountId> ProjectTipsApiServer<<Block as BlockT>::Hash, AccountId>
    for ProjectTips<C, Block>
where
    Block: BlockT,
    AccountId: Codec,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: ProjectTipsRuntimeApi<Block, AccountId>,
{
    fn get_evidence_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_evidence_period_end_block(at, project_id);
        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }
    fn get_staking_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_staking_period_end_block(at, project_id);
        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }
    fn get_drawing_period_end(
        &self,
        project_id: ProjectId,
        at: Option<Block::Hash>,
    ) -> RpcResult<(u64, u64, bool)> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_drawing_period_end(at, project_id);
        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn get_commit_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_commit_period_end_block(at, project_id);
        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn get_vote_period_end_block(
        &self,
        project_id: ProjectId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_vote_period_end_block(at, project_id);
        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn selected_as_juror(
        &self,
        project_id: ProjectId,
        who: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<bool> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.selected_as_juror(at, project_id, who);
        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }
}
