//! # Depends
//!
//! A library for ergonomic, performant, incremental computation between
//! arbitrary types.
//!
//! ## Why would I want that
//!
//! Most applications rely on some core logic which must respond to external
//! events. Often, the logic to transform each event in to an action is
//! straightforward, but as the application scales, many hard to reason-with
//! situations emerge from the combinatorial explosion of states.
//!
//! Dependency graphs are an excellent code architecture to tame complexity in
//! such scenarios.
//!
//! ```
//! use std::{collections::HashSet, rc::Rc};
//!
//! use depends::{
//!     core::{
//!         Dependency, Depends, HashValue, LeafNode, NodeHash, Resolve, UpdateDependee, UpdateLeaf,
//!     },
//!     derives::{dependencies, Dependee, Leaf},
//! };
//!
//! // A `Leaf` is a node which takes new values from outside the graph.
//! #[derive(Leaf, Default)]
//! pub struct NumberInput {
//!     value: i32,
//! }
//!
//! impl HashValue for NumberInput {
//!     fn hash_value(&self) -> NodeHash {
//!         NodeHash::Hashed(self.value as usize)
//!     }
//! }
//!
//! // `Leaf` types must provide a way for code outside to update their internal state.
//! // This is just a simple replace for now.
//! impl UpdateLeaf for NumberInput {
//!     type Input = i32;
//!
//!     fn update_mut(&mut self, input: Self::Input) {
//!         self.value = input;
//!     }
//! }
//!
//! // `dependencies` are derived to state what references `Dependee` nodes need to
//! // calculate their state on-demand. These could be any number of other `Dependee`s
//! // or `Leaf`s.
//! #[dependencies]
//! pub struct Components {
//!     left: LeafNode<NumberInput>,
//!     right: LeafNode<NumberInput>,
//! }
//!
//! // A `Dependee` i.e. its state is a pure transformation of other nodes
//! #[derive(Dependee, Default)]
//! #[depends(dependencies = Components)]
//! pub struct Sum {
//!     value: i32,
//! }
//!
//! impl HashValue for Sum {
//!     fn hash_value(&self) -> NodeHash {
//!         NodeHash::Hashed(self.value as usize)
//!     }
//! }
//!
//! // This trait specifies how a `Dependee` updates its internal state given its dependencies.
//! impl UpdateDependee for Sum {
//!     fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
//!         // `ComponentsRef` is auto-generated by `dependencies`. It's a read-reference
//!         // to each field of `Components`
//!         let ComponentsRef { left, right } = input;
//!         self.value = left.value + right.value;
//!     }
//! }
//!
//! struct MyGraph {
//!     left: Rc<LeafNode<NumberInput>>,
//!     right: Rc<LeafNode<NumberInput>>,
//!     // `SumNode` is auto-generated by `Dependee`.
//!     sum: Rc<SumNode>,
//! }
//!
//! // Compose a graph!
//! let left = NumberInput::default().into_leaf();
//! let right = NumberInput::default().into_leaf();
//! let sum = Sum::default().into_node(Components::new(Rc::clone(&left), Rc::clone(&right)));
//!
//! let graph = MyGraph { left, right, sum };
//!
//! // A `Visitor` is a collection which tracks which nodes have been visited each run.
//! let mut visitor = HashSet::<usize>::new();
//!
//! // Resolving the graph from any node will traverse via Depth First Search, prompting
//! // recalculation for any node whos dependencies have changed since last resolved.
//! assert_eq!(graph.sum.resolve_root(&mut visitor).value, 0);
//!
//! // update the leaves
//! graph.left.update(2);
//! graph.right.update(2);
//!
//! // We've successfully implemented simple addition! Only nodes which have dirty parents
//! // will be recalculated.
//! assert_eq!(graph.sum.resolve_root(&mut visitor).value, 4);
//! ```
//!
//! Clearly, to implement a simple addition problem, a dependency graph is
//! overkill. However, for more complex problems, where many inputs can change
//! and the output is a combination of many transformations on that input (and
//! derivations of it), `depends` can help you produce scalable, performant,
//! testable code out of the box.
pub mod core {
    pub use depends_core::execution::*;
}
pub mod derives {
    pub use depends_derives::*;
}

/// Visualisation tool for graphs.
#[cfg(feature = "graphviz")]
pub mod graphviz;
