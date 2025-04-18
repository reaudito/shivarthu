pub use super::*;
use frame_support::traits::{ReservableCurrency, VoteTally};
use pallet_referenda::{Deposit, TallyOf, TracksInfo};

pub trait ReferendumTrait<AccountId> {
	type Index: From<u32>
		+ Parameter
		+ Member
		+ Ord
		+ PartialOrd
		+ Copy
		+ HasCompact
		+ MaxEncodedLen;
	type Proposal: Parameter + Member + MaxEncodedLen;
	type ReferendumInfo: Eq + PartialEq + Debug + Encode + Decode + TypeInfo + Clone;
	type Preimages;
	type Call;
	type Moment;

	fn create_proposal(proposal_call: Self::Call) -> Self::Proposal;
	fn submit_proposal(caller: AccountId, proposal: Self::Proposal) -> Result<u32, DispatchError>;
	fn get_referendum_info(index: Self::Index) -> Option<Self::ReferendumInfo>;
	fn handle_referendum_info(infos: Self::ReferendumInfo) -> Option<ReferendumStates>;
	fn referendum_count() -> Self::Index;
	fn get_decision_period(index: Self::Index) -> Result<u128, DispatchError>;
}

pub trait ConvictionVotingTrait<AccountId> {
	type Vote;
	type AccountVote;
	type Index: From<u32>
		+ Parameter
		+ Member
		+ Ord
		+ PartialOrd
		+ Copy
		+ HasCompact
		+ MaxEncodedLen;
	type Balance;
	type Moment;

	fn u128_to_balance(x: u128) -> Option<Self::Balance>;
	fn vote_data(aye: bool, conviction: Conviction, balance: Self::Balance) -> Self::AccountVote;
	fn try_vote(
		caller: &AccountId,
		ref_index: Self::Index,
		vote: Self::AccountVote,
	) -> DispatchResult;
	fn try_remove_vote(caller: &AccountId, ref_index: Self::Index) -> DispatchResult;
}

impl<T: pallet_conviction_voting::Config<I>, I: 'static> ConvictionVotingTrait<AccountIdOf<T>>
	for pallet_conviction_voting::Pallet<T, I> where <<T as pallet_conviction_voting::Config<I>>::Polls as frame_support::traits::Polling<pallet_conviction_voting::Tally<<<T as pallet_conviction_voting::Config<I>>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance, <T as pallet_conviction_voting::Config<I>>::MaxTurnout>>>::Index: From<u32>
{
	type Vote = pallet_conviction_voting::VotingOf<T, I>;
	type AccountVote =
		pallet_conviction_voting::AccountVote<Self::Balance>;
	type Index = pallet_conviction_voting::PollIndexOf<T, I>;
	type Balance = <<T as pallet_conviction_voting::Config<I>>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type Moment = <T::BlockNumberProvider as BlockNumberProvider>::BlockNumber;

	fn vote_data(aye:bool, conviction: Conviction, balance: Self::Balance) -> Self::AccountVote {
		pallet_conviction_voting::AccountVote::Standard {
			vote: pallet_conviction_voting::Vote { aye, conviction },
			balance,
		}
	}
	fn try_vote(
		caller: &AccountIdOf<T>,
		ref_index: Self::Index,
		vote: Self::AccountVote,
	) -> DispatchResult {
		let origin = RawOrigin::Signed(caller.clone());
		pallet_conviction_voting::Pallet::<T, I>::vote(origin.into(), ref_index, vote)?;
		Ok(())
	}
	fn u128_to_balance(x: u128) -> Option<Self::Balance> {
		x.try_into().ok()
	}
	fn try_remove_vote(caller: &AccountIdOf<T>,ref_index: Self::Index) -> DispatchResult {
		let origin = RawOrigin::Signed(caller.clone());
		pallet_conviction_voting::Pallet::<T, I>::remove_vote(origin.into(),None,ref_index)?;
		Ok(())
	}
}

impl<T: frame_system::Config + pallet_referenda::Config<I>, I: 'static>
	ReferendumTrait<AccountIdOf<T>> for pallet_referenda::Pallet<T, I>
where
	<T as pallet_referenda::Config<I>>::RuntimeCall: Sync + Send,
{
	type Index = pallet_referenda::ReferendumIndex;
	type Proposal = Bounded<
		<T as pallet_referenda::Config<I>>::RuntimeCall,
		<T as frame_system::Config>::Hashing,
	>;
	type ReferendumInfo = pallet_referenda::ReferendumInfoOf<T, I>;
	type Preimages = <T as pallet_referenda::Config<I>>::Preimages;
	type Call = <T as frame_system::Config>::RuntimeCall;
	type Moment = <T::BlockNumberProvider as BlockNumberProvider>::BlockNumber;

	fn create_proposal(proposal_call: Self::Call) -> Self::Proposal {
		let call_formatted = <T as pallet_referenda::Config<I>>::RuntimeCall::from(proposal_call);
		let bounded_proposal = Self::Preimages::bound(call_formatted).expect("Operation failed");
		bounded_proposal
	}

	fn submit_proposal(
		who: AccountIdOf<T>,
		proposal: Self::Proposal,
	) -> Result<u32, DispatchError> {
		let enactment_moment = DispatchTime::After(0u32.into());
		let proposal_origin0 = RawOrigin::Root.into();
		let proposal_origin = Box::new(proposal_origin0);
		if let (Some(preimage_len), Some(proposal_len)) =
			(proposal.lookup_hash().and_then(|h| Self::Preimages::len(&h)), proposal.lookup_len())
		{
			if preimage_len != proposal_len {
				return Err(pallet_referenda::Error::<T, I>::PreimageStoredWithDifferentLength.into())
			}
		}
		let track = T::Tracks::track_for(&proposal_origin)
			.map_err(|_| pallet_referenda::Error::<T, I>::NoTrack)?;
		T::Currency::reserve(&who, T::SubmissionDeposit::get())?;
		let amount = T::SubmissionDeposit::get();
		let submission_deposit = Deposit { who, amount };
		let index = pallet_referenda::ReferendumCount::<T, I>::mutate(|x| {
			let r = *x;
			*x += 1;
			r
		});
		let now = T::BlockNumberProvider::current_block_number();
		let nudge_call =
			T::Preimages::bound(<<T as pallet_referenda::Config<I>>::RuntimeCall>::from(
				pallet_referenda::Call::nudge_referendum { index },
			))?;

		let alarm_interval = T::AlarmInterval::get().max(One::one());
		// Alarm must go off no earlier than `when`.
		// This rounds `when` upwards to the next multiple of `alarm_interval`.
		let when0 = now.saturating_add(T::UndecidingTimeout::get());
		let when = (when0.saturating_add(alarm_interval.saturating_sub(One::one())) /
			alarm_interval)
			.saturating_mul(alarm_interval);
		let result = T::Scheduler::schedule(
			DispatchTime::At(when),
			None,
			128u8,
			frame_system::RawOrigin::Root.into(),
			nudge_call,
		);
		if let Err(_e) = result {
			return Err(DispatchError::Other("SchedulerError"));
		}
		let alarm = result.ok().map(|x| (when, x));

		let status = pallet_referenda::ReferendumStatus {
			track,
			origin: *proposal_origin,
			proposal: proposal.clone(),
			enactment: enactment_moment,
			submitted: now,
			submission_deposit,
			decision_deposit: None,
			deciding: None,
			tally: TallyOf::<T, I>::new(track),
			in_queue: false,
			alarm,
		};
		pallet_referenda::ReferendumInfoFor::<T, I>::insert(
			index,
			Self::ReferendumInfo::Ongoing(status),
		);
		Ok(index)
	}

	fn get_referendum_info(index: Self::Index) -> Option<Self::ReferendumInfo> {
		pallet_referenda::ReferendumInfoFor::<T, I>::get(index)
	}
	fn handle_referendum_info(infos: Self::ReferendumInfo) -> Option<ReferendumStates> {
		match infos {
			Self::ReferendumInfo::Approved(..) => Some(ReferendumStates::Approved),
			Self::ReferendumInfo::Rejected(..) => Some(ReferendumStates::Rejected),
			Self::ReferendumInfo::Ongoing(..) => Some(ReferendumStates::Ongoing),
			_ => None,
		}
	}

	fn referendum_count() -> Self::Index {
		pallet_referenda::ReferendumCount::<T, I>::get()
	}

	fn get_decision_period(index: Self::Index) -> Result<u128, DispatchError> {
		let info = Self::get_referendum_info(index)
			.ok_or_else(|| DispatchError::Other("No referendum info found"))?;
		match info {
			Self::ReferendumInfo::Ongoing(ref info) => {
				let track_id = info.track;
				let track = T::Tracks::info(track_id)
					.ok_or_else(|| DispatchError::Other("No track info found"))?;
				let total_period = track.decision_period.saturating_add(track.prepare_period);
				// convert total_period to u128
				let total_period: u128 = total_period.try_into().map_err(|_| {
					DispatchError::Other("Failed to convert decision period to u128")
				})?;
				Ok(total_period)
			},
			_ => Err(DispatchError::Other("Not an ongoing referendum")),
		}
	}
}
