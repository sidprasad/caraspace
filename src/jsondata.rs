//! Relational JSON data model produced by [`crate::export_json_instance`].
//!
//! Every Rust value caraspace visualizes is flattened into this shape:
//! a flat list of [`IAtom`](crate::jsondata::IAtom) nodes plus a flat list of
//! [`IRelation`](crate::jsondata::IRelation) edges grouped by name. The same
//! shape is what spytial-core consumes on the JavaScript side — these structs
//! are part of the public, stable API.

use serde::Serialize;

/// A relational instance: the full set of atoms (nodes) and relations (edges)
/// extracted from a single Rust value.
///
/// Serialized as JSON in the HTML template and consumed by spytial-core's
/// `JSONDataInstance` constructor in the browser.
#[derive(Serialize, Debug)]
pub struct JsonDataInstance {
    /// All atoms (graph nodes), in serialization order.
    pub atoms: Vec<IAtom>,
    /// All relations (edges), grouped by relation name.
    pub relations: Vec<IRelation>,
}

/// A single atom — one node in the relational graph.
///
/// Atoms are created from struct instances, collection containers (sequence,
/// tuple, map), and primitive leaves. `id` is unique within the instance,
/// `type` is the Rust type name (e.g. `"Person"`, `"i32"`, `"sequence"`),
/// and `label` is the human-readable text shown in the diagram.
#[derive(Serialize, Debug)]
pub struct IAtom {
    /// Unique identifier within the enclosing [`JsonDataInstance`].
    pub id: String,
    /// Type name (e.g. struct name or `"sequence"`/`"tuple"`/`"map"`/`"i32"`).
    pub r#type: String,
    /// Human-readable label shown on the diagram.
    pub label: String,
}

/// A single tuple within a relation: the participating atoms and the type
/// of each position.
#[derive(Serialize, Debug)]
pub struct ITuple {
    /// Atom IDs in this tuple, in position order.
    pub atoms: Vec<String>,
    /// Type names of the atoms in this tuple, parallel to [`Self::atoms`].
    pub types: Vec<String>,
}

/// A relation — a named, typed edge set. All tuples in a relation share
/// the same arity and position types.
///
/// Examples: a field relation `name(Person, string)`, a sequence relation
/// `idx(sequence, index, T)`, or a map relation `map_entry(map, K, V)`.
#[derive(Serialize, Debug)]
pub struct IRelation {
    /// Stable identifier for the relation (currently the same as [`Self::name`]).
    pub id: String,
    /// Relation name — `"idx"`, `"map_entry"`, or a struct field name.
    pub name: String,
    /// Type names for each position in the tuples, in position order.
    pub types: Vec<String>,
    /// All tuples belonging to this relation.
    pub tuples: Vec<ITuple>,
}
