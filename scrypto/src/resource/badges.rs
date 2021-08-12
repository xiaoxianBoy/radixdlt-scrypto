use sbor::{Decode, Describe, Encode};

use crate::kernel::*;
use crate::resource::*;
use crate::types::*;

/// A bucket that holds badge resource.
#[derive(Debug, Describe, Encode, Decode)]
pub struct Badges {
    bid: BID,
}

impl From<BID> for Badges {
    fn from(bid: BID) -> Self {
        Self { bid }
    }
}

impl Into<BID> for Badges {
    fn into(self) -> BID {
        self.bid
    }
}

impl Badges {
    pub fn put(&mut self, other: Self) {
        let input = CombineBucketsInput {
            bucket: self.bid,
            other: other.bid,
        };
        let _: CombineBucketsOutput = call_kernel(COMBINE_BUCKETS, input);
    }

    pub fn take(&mut self, amount: U256) -> Self {
        let input = SplitBucketInput {
            bucket: self.bid,
            amount,
        };
        let output: SplitBucketOutput = call_kernel(SPLIT_BUCKET, input);

        output.bucket.into()
    }

    pub fn borrow(&self) -> BadgesRef {
        let input = BorrowImmutableInput { bucket: self.bid };
        let output: BorrowImmutableOutput = call_kernel(BORROW_IMMUTABLE, input);

        output.reference.into()
    }

    pub fn amount(&self) -> U256 {
        let input = GetAmountInput { bucket: self.bid };
        let output: GetAmountOutput = call_kernel(GET_AMOUNT, input);

        output.amount
    }

    pub fn resource(&self) -> Address {
        let input = GetResourceInput { bucket: self.bid };
        let output: GetResourceOutput = call_kernel(GET_RESOURCE, input);

        output.resource
    }
}