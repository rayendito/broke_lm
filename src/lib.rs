pub mod trie;
pub mod train_utils;
pub mod query_utils;

// exposing Trie to the outside world
// so that we don't have to do crate::trie::Trie, we only do crate::Trie
pub use trie::Trie;
