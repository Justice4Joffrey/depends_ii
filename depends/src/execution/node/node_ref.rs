use std::cell::Ref;
use std::hash::Hasher;
use crate::{HashValue, NodeHash};

use super::NodeState;

/// Short-hand for the output read-reference of a node.
pub type NodeRef<'a, T> = Ref<'a, NodeState<T>>;
