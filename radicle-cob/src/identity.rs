// Copyright © 2022 The Radicle Link Contributors
//
// This file is part of radicle-link, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

use git_ext::Oid;

/// An [`Identity`] represents a content addressed identity
/// (i.e. expected to be stored in a git backend).
///
/// It should have:
///   * A delegate system
///   * A content addressable identifier
///   * A unique, stable identifier
pub trait Identity {
    type Identifier;

    /// Confirm that the given [`crypto::PublicKey`] is a delegate for
    /// the identity.
    fn is_delegate(&self, delegation: &crypto::PublicKey) -> bool;

    /// Provide the content address for the given identity. This is
    /// expected to be the latest address for the identity at the time
    /// of use.
    fn content_id(&self) -> Oid;
}
