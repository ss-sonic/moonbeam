//! A set of constant values used in substrate runtime.

pub mod mb_genesis {
	use node_primitives::{Balance};
	use super::currency::{GLMR};
	/// 8 decimals for 500_000 glmr units
	pub const TOTAL_GLMR_SUPPLY: Balance = 10_000_000 * GLMR;
	pub const TREASURY_FUND: Balance = TOTAL_GLMR_SUPPLY / 5;
	pub const REWARD_PER_YEAR: Balance = 250_000 * GLMR;
	
	pub const VALIDATORS_PER_SESSION: u8 = 2;
}

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const MILLICENTS: Balance = 1_000_000_000;
	pub const CENTS: Balance = 1_000 * MILLICENTS;    // assume this is worth about a cent.
	pub const DOLLARS: Balance = 100 * CENTS;

	pub const MILLIGLMR: Balance = 1_000;
	pub const CENTIGLMR: Balance = 1_000 * MILLIGLMR;
	pub const GLMR: Balance = 100 * CENTIGLMR;
}

/// Time.
pub mod time {
	use node_primitives::{Moment, BlockNumber};

	/// Since BABE is probabilistic this is the average expected block time that
	/// we are targetting. Blocks will be produced at a minimum duration defined
	/// by `SLOT_DURATION`, but some slots will not be allocated to any
	/// authority and hence no block will be produced. We expect to have this
	/// block time on average following the defined slot duration and the value
	/// of `c` configured for BABE (where `1 - c` represents the probability of
	/// a slot being empty).
	/// This value is only used indirectly to define the unit constants below
	/// that are expressed in blocks. The rest of the code should use
	/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
	/// minimum period).
	///
	/// If using BABE with secondary slots (default) then all of the slots will
	/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
	/// `SLOT_DURATION` should have the same value.
	///
	/// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
	pub const MILLISECS_PER_BLOCK: Moment = 3000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	// Sessions are set low (1 per minute) for debugging purposes.
	// As a block is produced every 3 seconds by configuration, a new session will occur every X blocks.
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 5 * MINUTES;
	pub const EPOCH_DURATION_IN_SLOTS: u64 = {
		const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

		(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
	};

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// epochs/session per era
	pub const EPOCH_PER_ERA: u8 = 1;

	// convenience year in milliseconds. TODO.
	pub const MILLISECS_PER_YEAR: u64 = 3660 * 24 * 365 * 1000;
}
