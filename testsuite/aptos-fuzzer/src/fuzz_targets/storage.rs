// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{corpus_from_strategy, fuzz_data_to_value, FuzzTargetImpl};
use aptos_accumulator::test_helpers::{
    arb_hash_batch, arb_list_of_hash_batches, arb_three_hash_batches, arb_two_hash_batches,
    test_append_empty_impl, test_append_many_impl, test_consistency_proof_impl,
    test_get_frozen_subtree_hashes_impl, test_proof_impl, test_range_proof_impl,
};
use aptos_crypto::HashValue;
use aptos_jellyfish_merkle::test_helper::{
    arb_existent_kvs_and_nonexistent_keys, arb_kv_pair_with_distinct_last_nibble,
    arb_tree_with_index, test_get_leaf_count, test_get_range_proof, test_get_with_proof,
    test_get_with_proof_with_distinct_last_nibble,
};
use aptos_proptest_helpers::ValueGenerator;
use aptos_types::state_store::state_key::StateKey;
use aptosdb::{
    schema::fuzzing::fuzz_decode,
    test_helper::{arb_blocks_to_commit, test_save_blocks_impl},
};
use proptest::{
    collection::{hash_set, vec},
    prelude::*,
};
use scratchpad::test_utils::proptest_helpers::{
    arb_smt_correctness_case, test_smt_correctness_impl,
};

#[derive(Clone, Debug, Default)]
pub struct StorageSaveBlocks;

impl FuzzTargetImpl for StorageSaveBlocks {
    fn description(&self) -> &'static str {
        "Storage save blocks"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_blocks_to_commit()))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_blocks_to_commit());
        let threshold = fuzz_data_to_value(data, 10..20usize);
        test_save_blocks_impl(input, threshold);
    }
}

#[derive(Clone, Debug, Default)]
pub struct StorageSchemaDecode;

impl FuzzTargetImpl for StorageSchemaDecode {
    fn description(&self) -> &'static str {
        "Storage schemas do not panic on corrupted bytes."
    }

    fn generate(&self, _idx: usize, gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(gen.generate(prop_oneof![
            100 => vec(any::<u8>(), 0..1024),
            1 => vec(any::<u8>(), 1024..1024 * 10),
        ]))
    }

    fn fuzz(&self, data: &[u8]) {
        fuzz_decode(data)
    }
}

//============== JellyfishMerkleTree =============

#[derive(Clone, Debug, Default)]
pub struct JellyfishGetWithProof;

impl FuzzTargetImpl for JellyfishGetWithProof {
    fn description(&self) -> &'static str {
        "JellyfishMerkleTree get with proof"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(
            arb_existent_kvs_and_nonexistent_keys::<StateKey>(1000, 100),
        ))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(
            data,
            arb_existent_kvs_and_nonexistent_keys::<StateKey>(1000, 100),
        );
        test_get_with_proof(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct JellyfishGetWithProofWithDistinctLastNibble;

impl FuzzTargetImpl for JellyfishGetWithProofWithDistinctLastNibble {
    fn description(&self) -> &'static str {
        "JellyfishMerkleTree get with proof with distinct last nibble"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(
            arb_kv_pair_with_distinct_last_nibble::<StateKey>(),
        ))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_kv_pair_with_distinct_last_nibble::<StateKey>());
        test_get_with_proof_with_distinct_last_nibble(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct JellyfishGetRangeProof;

impl FuzzTargetImpl for JellyfishGetRangeProof {
    fn description(&self) -> &'static str {
        "JellyfishMerkleTree get range proof"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_tree_with_index::<StateKey>(100)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_tree_with_index::<StateKey>(100));
        test_get_range_proof(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct JellyfishGetLeafCount;

impl FuzzTargetImpl for JellyfishGetLeafCount {
    fn description(&self) -> &'static str {
        "JellyfishMerkleTree get leaf count"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(hash_set(any::<HashValue>(), 1..1000)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, hash_set(any::<HashValue>(), 1..1000));
        test_get_leaf_count(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AccumulatorFrozenSubtreeHashes;

impl FuzzTargetImpl for AccumulatorFrozenSubtreeHashes {
    fn description(&self) -> &'static str {
        "Accumulator frozen subtree hashes"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_hash_batch(1000)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_hash_batch(1000));
        test_get_frozen_subtree_hashes_impl(input);
    }
}

//============== Scratchpad =============

#[derive(Clone, Debug, Default)]
pub struct SparseMerkleCorrectness;

impl FuzzTargetImpl for SparseMerkleCorrectness {
    fn description(&self) -> &'static str {
        "Scratchpad SMT correctness."
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_smt_correctness_case()))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_smt_correctness_case());
        test_smt_correctness_impl(input)
    }
}

//============== Accumulator =============

#[derive(Clone, Debug, Default)]
pub struct AccumulatorProof;

impl FuzzTargetImpl for AccumulatorProof {
    fn description(&self) -> &'static str {
        "Accumulator proof"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_two_hash_batches(100)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_two_hash_batches(100));
        test_proof_impl(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AccumulatorConsistencyProof;

impl FuzzTargetImpl for AccumulatorConsistencyProof {
    fn description(&self) -> &'static str {
        "Accumulator consistency proof"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_two_hash_batches(100)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_two_hash_batches(100));
        test_consistency_proof_impl(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AccumulatorRangeProof;

impl FuzzTargetImpl for AccumulatorRangeProof {
    fn description(&self) -> &'static str {
        "Accumulator range proof"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_three_hash_batches(100)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_three_hash_batches(100));
        test_range_proof_impl(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AccumulatorAppendMany;

impl FuzzTargetImpl for AccumulatorAppendMany {
    fn description(&self) -> &'static str {
        "Accumulator amend many leaves"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_list_of_hash_batches(10, 10)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_list_of_hash_batches(10, 10));
        test_append_many_impl(input);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AccumulatorAppendEmpty;

impl FuzzTargetImpl for AccumulatorAppendEmpty {
    fn description(&self) -> &'static str {
        "Accumulator amend empty leave"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        Some(corpus_from_strategy(arb_hash_batch(100)))
    }

    fn fuzz(&self, data: &[u8]) {
        let input = fuzz_data_to_value(data, arb_hash_batch(100));
        test_append_empty_impl(input);
    }
}
