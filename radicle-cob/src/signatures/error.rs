// Copyright © 2019-2020 The Radicle Foundation <hello@radicle.foundation>
//
// This file is part of radicle-link, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

use radicle_crypto::ssh::ExtendedSignatureError;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Signature {
    #[error("missing {0}")]
    Missing(&'static str),

    #[error(transparent)]
    Serde(#[from] serde::de::value::Error),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Signatures {
    #[error(transparent)]
    ExtendedSignature(#[from] ExtendedSignatureError),

    #[error(transparent)]
    Signature(#[from] Signature),
}
