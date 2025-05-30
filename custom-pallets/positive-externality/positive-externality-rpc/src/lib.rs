use codec::Codec;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::{ErrorObject, ErrorObjectOwned},
};
use positive_externality_runtime_api::PositiveExternalityApi as PositiveExternalityRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

#[rpc(client, server)]
pub trait PositiveExternalityApi<BlockHash, AccountId> {
    #[method(name = "positiveexternality_evidenceperiodendblock")]
    fn get_evidence_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "positiveexternality_stakingperiodendblock")]
    fn get_staking_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "positiveexternality_drawingperiodend")]
    fn get_drawing_period_end(
        &self,
        user_to_calculate: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<(u64, u64, bool)>;
    #[method(name = "positiveexternality_commitendblock")]
    fn get_commit_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "positiveexternality_voteendblock")]
    fn get_vote_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<u32>>;
    #[method(name = "positiveexternality_selectedjuror")]
    fn selected_as_juror(
        &self,
        user_to_calculate: AccountId,
        who: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<bool>;
    #[method(name = "positiveexternality_postbyaddresslength")]
    fn post_by_address_length(&self, user: AccountId, at: Option<BlockHash>) -> RpcResult<u64>;

    #[method(name = "positiveexternality_paginateposts")]
    fn paginate_posts_by_address(
        &self,
        user: AccountId,
        page: u64,
        page_size: u64,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u64>>>;

    #[method(name = "positiveexternality_paginateposts_latest")]
    fn paginate_posts_by_address_latest(
        &self,
        user: AccountId,
        page: u64,
        page_size: u64,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u64>>>;

    #[method(name = "positiveexternality_validationlistlength")]
    fn validation_list_length(&self, at: Option<BlockHash>) -> RpcResult<u64>;

    #[method(name = "positiveexternality_validationlist_latest")]
    fn validation_list_latest(
        &self,
        page: u64,
        page_size: u64,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<AccountId>>>;

    #[method(name = "positiveexternality_has_user_staked")]
    fn has_user_staked(
        &self,
        user_to_calculate: AccountId,
        who: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<bool>;

    #[method(name = "positiveexternality_user_staked_value")]
    fn user_staked_value(
        &self,
        user_to_calculate: AccountId,
        who: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<u64>;

    #[method(name = "positiveexternality_paginateall_posts")]
    fn paginate_all_post(
        &self,
        page: u64,
        page_size: u64,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u64>>>;

    #[method(name = "all_postlength")]
    fn all_post_length(&self, at: Option<BlockHash>) -> RpcResult<u64>;
}

/// A struct that implements the `SumStorageApi`.
pub struct PositiveExternality<C, M> {
    // If you have more generics, no need to SumStorage<C, M, N, P, ...>
    // just use a tuple like SumStorage<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> PositiveExternality<C, M> {
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

impl<C, Block, AccountId> PositiveExternalityApiServer<<Block as BlockT>::Hash, AccountId>
    for PositiveExternality<C, Block>
where
    Block: BlockT,
    AccountId: Codec,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: PositiveExternalityRuntimeApi<Block, AccountId>,
{
    fn get_evidence_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_evidence_period_end_block(at, user_to_calculate);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }
    fn get_staking_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }

        let runtime_api_result = api.get_staking_period_end_block(at, user_to_calculate);

        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }
    fn get_drawing_period_end(
        &self,
        user_to_calculate: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<(u64, u64, bool)> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_drawing_period_end(at, user_to_calculate);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn get_commit_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_commit_period_end_block(at, user_to_calculate);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn get_vote_period_end_block(
        &self,
        user_to_calculate: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<u32>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.get_vote_period_end_block(at, user_to_calculate);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn selected_as_juror(
        &self,
        user_to_calculate: AccountId,
        who: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<bool> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.selected_as_juror(at, user_to_calculate, who);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn post_by_address_length(&self, user: AccountId, at: Option<Block::Hash>) -> RpcResult<u64> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.post_by_address_length(at, user);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn paginate_posts_by_address(
        &self,
        user: AccountId,
        page: u64,
        page_size: u64,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<Vec<u64>>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.paginate_posts_by_address(at, user, page, page_size);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn paginate_posts_by_address_latest(
        &self,
        user: AccountId,
        page: u64,
        page_size: u64,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<Vec<u64>>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.paginate_posts_by_address_latest(at, user, page, page_size);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn validation_list_length(&self, at: Option<Block::Hash>) -> RpcResult<u64> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.validation_list_length(at);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn validation_list_latest(
        &self,
        page: u64,
        page_size: u64,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<Vec<AccountId>>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.validation_list_latest(at, page, page_size);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn has_user_staked(
        &self,
        user_to_calculate: AccountId,
        who: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<bool> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.has_user_staked(at, user_to_calculate, who);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn user_staked_value(
        &self,
        user_to_calculate: AccountId,
        who: AccountId,
        at: Option<Block::Hash>,
    ) -> RpcResult<u64> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.user_staked_value(at, user_to_calculate, who);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn paginate_all_post(
        &self,
        page: u64,
        page_size: u64,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<Vec<u64>>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.paginate_all_post(at, page, page_size);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }

    fn all_post_length(&self, at: Option<Block::Hash>) -> RpcResult<u64> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

        let runtime_api_result = api.all_post_length(at);

        fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
            ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
        }
        let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
        Ok(res)
    }
}
