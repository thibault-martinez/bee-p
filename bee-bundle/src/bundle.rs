use crate::{Transaction, TransactionBuilder};
use crypto::Sponge;
use std::marker::PhantomData;
use ternary::TritsBuf;

#[derive(Default)]
pub struct Transactions(Vec<Transaction>);

impl Transactions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, transaction: Transaction) {
        self.0.push(transaction);
    }
}

#[derive(Default)]
pub struct TransactionBuilders(Vec<TransactionBuilder>);

// TODO should be in tx module ?
impl TransactionBuilders {
    pub fn push(&mut self, transaction_builder: TransactionBuilder) {
        self.0.push(transaction_builder);
    }
}

///  Bundles

pub struct Bundle {
    transactions: Transactions,
}

impl Bundle {
    pub fn transactions(&self) -> &Transactions {
        &self.transactions
    }

    pub fn len(&self) -> usize {
        self.transactions.0.len()
    }
}

/// Incoming bundles

struct IncomingBundleBuilder {
    builders: TransactionBuilders,
}

impl IncomingBundleBuilder {
    pub fn push(&mut self, transaction_builder: TransactionBuilder) -> &mut Self {
        self.builders.push(transaction_builder);
        self
    }
}

/// Outgoing bundles

#[derive(Debug)]
pub enum OutgoingBundleBuilderError {}

trait OutgoingBundleBuilderStage {}

#[derive(Default)]
pub struct Raw;
impl OutgoingBundleBuilderStage for Raw {}

pub struct Sealed;
impl OutgoingBundleBuilderStage for Sealed {}

pub struct Signed;
impl OutgoingBundleBuilderStage for Signed {}

pub struct Attached;
impl OutgoingBundleBuilderStage for Attached {}

pub struct Validated;
impl OutgoingBundleBuilderStage for Validated {}

#[derive(Default)]
pub struct StagedOutgoingBundleBuilder<E, H, S> {
    builders: TransactionBuilders,
    essence_sponge: PhantomData<E>,
    hash_sponge: PhantomData<H>,
    stage: PhantomData<S>,
}

pub type OutgoingBundleBuilderSponge<E, H> = StagedOutgoingBundleBuilder<E, H, Raw>;
// TODO default to Kerl
pub type OutgoingBundleBuilder = OutgoingBundleBuilderSponge<crypto::CurlP81, crypto::CurlP81>;

impl<E, H, S> StagedOutgoingBundleBuilder<E, H, S>
where
    E: Sponge + Default,
    H: Sponge + Default,
    S: OutgoingBundleBuilderStage,
{
    pub fn calculate_hash(&self) -> TritsBuf {
        let mut sponge = E::default();

        for builder in &self.builders.0 {
            // TODO sponge.absorb(builder.essence());
        }

        sponge.squeeze()
    }
}

impl<E, H> StagedOutgoingBundleBuilder<E, H, Raw>
where
    E: Sponge + Default,
    H: Sponge + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, builder: TransactionBuilder) {
        self.builders.push(builder);
    }

    pub fn seal(
        self,
    ) -> Result<StagedOutgoingBundleBuilder<E, H, Sealed>, OutgoingBundleBuilderError> {
        // TODO Impl
        Ok(StagedOutgoingBundleBuilder::<E, H, Sealed> {
            builders: self.builders,
            essence_sponge: PhantomData,
            hash_sponge: PhantomData,
            stage: PhantomData,
        })
    }
}

impl<E, H> StagedOutgoingBundleBuilder<E, H, Sealed>
where
    E: Sponge + Default,
    H: Sponge + Default,
{
    pub fn sign(
        self,
    ) -> Result<StagedOutgoingBundleBuilder<E, H, Signed>, OutgoingBundleBuilderError> {
        // TODO Impl
        Ok(StagedOutgoingBundleBuilder::<E, H, Signed> {
            builders: self.builders,
            essence_sponge: PhantomData,
            hash_sponge: PhantomData,
            stage: PhantomData,
        })
    }
}

impl<E, H> StagedOutgoingBundleBuilder<E, H, Signed>
where
    E: Sponge + Default,
    H: Sponge + Default,
{
    pub fn attach(
        self,
    ) -> Result<StagedOutgoingBundleBuilder<E, H, Attached>, OutgoingBundleBuilderError> {
        // TODO Impl
        Ok(StagedOutgoingBundleBuilder::<E, H, Attached> {
            builders: self.builders,
            essence_sponge: PhantomData,
            hash_sponge: PhantomData,
            stage: PhantomData,
        })
    }
}

impl<E, H> StagedOutgoingBundleBuilder<E, H, Attached>
where
    E: Sponge + Default,
    H: Sponge + Default,
{
    pub fn validate(
        self,
    ) -> Result<StagedOutgoingBundleBuilder<E, H, Validated>, OutgoingBundleBuilderError> {
        // TODO Impl
        Ok(StagedOutgoingBundleBuilder::<E, H, Validated> {
            builders: self.builders,
            essence_sponge: PhantomData,
            hash_sponge: PhantomData,
            stage: PhantomData,
        })
    }
}

impl<E, H> StagedOutgoingBundleBuilder<E, H, Validated>
where
    E: Sponge + Default,
    H: Sponge + Default,
{
    pub fn build(self) -> Result<Bundle, OutgoingBundleBuilderError> {
        // TODO Impl
        let mut transactions = Transactions::new();

        for transaction_builder in self.builders.0 {
            transactions.push(transaction_builder.build());
        }

        Ok(Bundle {
            transactions: transactions,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_test() -> Result<(), OutgoingBundleBuilderError> {
        let mut bundle_builder = OutgoingBundleBuilder::new();

        for _ in 0..5 {
            bundle_builder.push(TransactionBuilder::default());
        }

        let bundle = bundle_builder
            .seal()?
            .sign()?
            .attach()?
            .validate()?
            .build()?;

        assert_eq!(bundle.len(), 5);

        Ok(())
    }
}
