//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "transformer-chain-diagram",
        "src/patterns/transformer_chain/transformer_chain.svg"
)))]
//! Provides arbitrarily chained groups of
//! [`crate::primitives::transformer::Transformer`]s to be setup and used
//! repeatedly.
//!
//! ![transformer chain diagram][transformer-chain-diagram]
pub mod heap;
