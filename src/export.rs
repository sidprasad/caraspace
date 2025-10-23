//! # Semantic-Aware Rust Data Export to Relational JSON
//! 
//! This module implements a custom Serde serializer that preserves the semantic
//! distinctions between different Rust data structures, going beyond generic JSON
//! to create rich relational representations suitable for graph visualization.
//! 
//! ## Core Design Philosophy
//! 
//! **Problem**: Standard JSON serialization loses semantic information:
//! - Structs and Maps both become `{"key": "value"}` 
//! - Arrays and Tuples both become `[item1, item2]`
//! - No distinction between positional vs associative vs named access patterns
//! 
//! **Solution**: Use Serde's semantic entry points to preserve collection intent:
//! - `serialize_struct` vs `serialize_map` - different access semantics  
//! - `serialize_seq` vs `serialize_tuple` - different position meanings
//! - Field names vs map keys vs array indices - different relationship types
//! 
//! ## Relationalization Patterns
//! 
//! | Rust Type | Relation Pattern | Reasoning |
//! |-----------|------------------|-----------|
//! | `struct Foo { x: T }` | `x(foo_instance, value)` | Field names are semantic roles |
//! | `Vec<T>`, `[T; N]` | `idx(container, "0", elem)` | Stable positional access |
//! | `(T1, T2, T3)` | `idx(tuple, "0", elem)` | Fixed positional semantics |
//! | `HashMap<K,V>` | `map_entry(map, key, val)` | Associative key→value lookup |
//! | `Point(x, y)` | `idx(point, "0", x_coord)` | Named but positional fields |
//! 
//! ## Visualization Benefits
//! 
//! This semantic preservation enables CnD layout specifications that understand
//! the data structure intent:
//! 
//! ```yaml
//! # Position arrays in grid layout
//! idx: 
//!   - when: tuple[0].type == "sequence"  
//!     position: grid_layout
//! 
//! # Use struct field semantics for labeling
//! name:
//!   - position: as_label
//! position: 
//!   - layout: coordinate_system
//! 
//! # Maps get key-value pair layout
//! map_entry:
//!   - layout: association_arrows
//! ```
//! 
//! ## Type System Integration
//! 
//! - **Struct names** become atom types (not generic "struct")
//! - **Primitive types** preserved exactly (`i32`, `f64`, `string`)
//! - **Collection types** use semantic names (`sequence`, `tuple`, `map`)
//! - **Relations** carry type information for both ends of relationships

use crate::jsondata::*;
use crate::cnd_annotations::SpytialDecorators;
use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};
use serde::ser;
use std::collections::HashMap;
use std::fmt;

/// Export a Rust data structure to our JSON instance format using custom Serde serialization
pub fn export_json_instance<T: Serialize>(value: &T) -> JsonDataInstance {
    let mut serializer = JsonDataSerializer::new();
    value.serialize(&mut serializer).unwrap();
    JsonDataInstance {
        atoms: serializer.atoms,
        relations: serializer.relations.into_values().collect(),
    }
}

/// Export a Rust data structure and collect CnD decorators from all encountered types
/// Excludes the root type from collection to avoid double-counting
pub fn export_json_instance_with_decorators<T: Serialize>(value: &T, root_type_name: &str) -> (JsonDataInstance, SpytialDecorators) {
    let mut serializer = JsonDataSerializer::new();
    serializer.exclude_type = Some(root_type_name.to_string());
    value.serialize(&mut serializer).unwrap();
    let instance = JsonDataInstance {
        atoms: serializer.atoms,
        relations: serializer.relations.into_values().collect(),
    };
    (instance, serializer.collected_decorators)
}

/// Custom Serde serializer that preserves semantic structure for different collection types
pub struct JsonDataSerializer {
    counter: usize,
    atoms: Vec<IAtom>,
    relations: HashMap<String, IRelation>,
    collected_decorators: SpytialDecorators,
    visited_types: std::collections::HashSet<String>,
    exclude_type: Option<String>,
    /// Cache for singleton atoms (like None, unit, etc.) that should be reused
    singleton_atoms: HashMap<(String, String), String>,  // (type, label) -> atom_id
}

impl JsonDataSerializer {
    fn new() -> Self {
        Self {
            counter: 0,
            atoms: vec![],
            relations: HashMap::new(),
            collected_decorators: SpytialDecorators::default(),
            visited_types: std::collections::HashSet::new(),
            exclude_type: None,
            singleton_atoms: HashMap::new(),
        }
    }

    fn fresh_id(&mut self) -> String {
        let id = format!("atom{}", self.counter);
        self.counter += 1;
        id
    }

    fn emit_atom(&mut self, typ: &str, label: &str) -> String {
        let id = self.fresh_id();
        self.atoms.push(IAtom {
            id: id.clone(),
            r#type: typ.to_string(),
            label: label.to_string(),
        });
        id
    }

    /// Get or create a singleton atom - atoms that should only exist once
    /// (like None, unit, true, false, etc.)
    fn get_or_create_singleton(&mut self, typ: &str, label: &str) -> String {
        let key = (typ.to_string(), label.to_string());
        
        if let Some(existing_id) = self.singleton_atoms.get(&key) {
            return existing_id.clone();
        }
        
        // Create new singleton atom
        let id = self.emit_atom(typ, label);
        self.singleton_atoms.insert(key, id.clone());
        id
    }

    fn push_relation(&mut self, name: &str, atoms: Vec<String>, types: Vec<&str>) {
        let types: Vec<String> = types.iter().map(|s| s.to_string()).collect();
        let tuple = ITuple { atoms, types: types.clone() };

        let rel = self.relations.entry(name.to_string()).or_insert(IRelation {
            id: name.to_string(),
            name: name.to_string(),
            types,
            tuples: vec![],
        });
        rel.tuples.push(tuple);
    }

    /// Collect decorators for a struct type if it hasn't been visited yet
    /// This method now attempts automatic registration for types that might have decorators
    fn collect_decorators_for_type(&mut self, type_name: &str) {
        // Skip if this type should be excluded
        if let Some(ref exclude) = self.exclude_type {
            if type_name == exclude {
                return;
            }
        }
        
        // Only collect decorators once per type to avoid duplicates
        if self.visited_types.contains(type_name) {
            return;
        }
        self.visited_types.insert(type_name.to_string());

        // First, try to get already-registered decorators
        if let Some(type_decorators) = crate::cnd_annotations::get_type_decorators(type_name) {
            // Merge the decorators into our collected set
            self.collected_decorators.constraints.extend(type_decorators.constraints);
            self.collected_decorators.directives.extend(type_decorators.directives);
            return;
        }
        
        // If not found, try to trigger registration by calling known decorated type methods
        // This is a heuristic approach: we try to trigger registration for common patterns
        if self.try_trigger_registration(type_name) {
            // After triggering, try to get decorators again
            if let Some(type_decorators) = crate::cnd_annotations::get_type_decorators(type_name) {
                self.collected_decorators.constraints.extend(type_decorators.constraints);
                self.collected_decorators.directives.extend(type_decorators.directives);
            }
        }
    }
    
    /// Attempt to trigger registration for a type name by trying common patterns
    /// This is a heuristic approach that works for types that are already linked
    fn try_trigger_registration(&self, _type_name: &str) -> bool {
        // In a real implementation, this could use reflection or other mechanisms
        // For now, we'll rely on the fact that calling decorators() on any decorated type
        // should register that type. But since we can't call trait methods by string name,
        // this is limited.
        
        // This is where we could add specific registration calls for common types
        // or use a more sophisticated discovery mechanism
        false
    }
}

#[derive(Debug)]
pub struct SerializationError(String);

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Serialization error: {}", self.0)
    }
}

impl std::error::Error for SerializationError {}

impl ser::Error for SerializationError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerializationError(msg.to_string())
    }
}

impl<'a> Serializer for &'a mut JsonDataSerializer {
    type Ok = String; // Return the atom ID
    type Error = SerializationError;

    // Collection serializers with proper semantics
    type SerializeSeq = SequenceSerializer<'a>;
    type SerializeTuple = TupleSerializer<'a>;
    type SerializeTupleStruct = TupleStructSerializer<'a>;
    type SerializeTupleVariant = TupleVariantSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = StructSerializer<'a>;
    type SerializeStructVariant = StructVariantSerializer<'a>;

    // Primitive types
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        // Booleans are singletons - there's only one true and one false
        Ok(self.get_or_create_singleton("bool", &v.to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("i8", &v.to_string()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("i16", &v.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("i32", &v.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("i64", &v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("u8", &v.to_string()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("u16", &v.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("u32", &v.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("u64", &v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("f32", &v.to_string()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("f64", &v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("char", &v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("string", v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(self.emit_atom("bytes", &format!("{:?}", v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.get_or_create_singleton("None", "None"))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // Unit () is a singleton
        Ok(self.get_or_create_singleton("unit", "()"))
    }

    fn serialize_unit_struct(self, name: &str) -> Result<Self::Ok, Self::Error> {
        // Unit structs are singletons - only one instance of each unit struct type exists
        Ok(self.get_or_create_singleton("unit_struct", name))
    }

    fn serialize_unit_variant(
        self,
        enum_name: &str,
        _variant_index: u32,
        variant: &str,
    ) -> Result<Self::Ok, Self::Error> {
        // Unit variants are singletons - Color::Red is always the same value
        // This is similar to None, (), true, false - zero-sized types with no data
        Ok(self.get_or_create_singleton(enum_name, variant))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let struct_id = self.emit_atom("newtype_struct", name);
        let inner_id = value.serialize(&mut *self)?;
        self.push_relation("value", vec![struct_id.clone(), inner_id], vec!["newtype_struct", "atom"]);
        Ok(struct_id)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        enum_name: &str,
        _variant_index: u32,
        variant: &str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let variant_id = self.emit_atom(enum_name, variant);
        let inner_id = value.serialize(&mut *self)?;
        self.push_relation("variant_value", vec![variant_id.clone(), inner_id], vec![enum_name, "atom"]);
        Ok(variant_id)
    }

    /// ## INDEXABLE SEQUENCES - `idx(container, position, element)`
    /// 
    /// **Decision**: Use positional indexing for collections with stable, meaningful positions
    /// and O(1) random access semantics. These are the "array-like" collections where position
    /// is intrinsic to the data structure's semantics.
    /// 
    /// **Covers**: `Vec<T>`, `[T; N]`, slices, `VecDeque<T>` (when used as indexed container)
    /// 
    /// **Relationalization**: `idx(container_id, position_string, element_id)`
    /// - Position is serialized as string for consistency with other relation keys
    /// - Preserves O(1) access semantics in the relational model
    /// - Enables CnD layouts based on sequential positioning
    /// 
    /// **Example**: `vec![1, 2, 3]` becomes:
    /// ```
    /// idx: ["vec_id", "0", "elem1"] -> ["sequence", "index", "i32"]
    /// idx: ["vec_id", "1", "elem2"] -> ["sequence", "index", "i32"]  
    /// idx: ["vec_id", "2", "elem3"] -> ["sequence", "index", "i32"]
    /// ```
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let seq_id = self.emit_atom("sequence", &format!("seq[{}]", len.unwrap_or(0)));
        Ok(SequenceSerializer {
            serializer: self,
            seq_id,
            index: 0,
        })
    }

    /// ## HETEROGENEOUS TUPLES - `idx(container, position, element)`
    /// 
    /// **Decision**: Use positional indexing for tuples since they have fixed arity and
    /// heterogeneous types where position has semantic meaning (like coordinates, pairs).
    /// 
    /// **Covers**: `(T1, T2, ...)`, `std::tuple` types
    /// 
    /// **Relationalization**: Same as sequences but with "tuple" type
    /// - Position maps to semantic roles (0=x, 1=y for coordinates)
    /// - Fixed arity known at compile time
    /// - Heterogeneous element types preserved
    /// 
    /// **Example**: `("hello", 42, true)` becomes:
    /// ```
    /// idx: ["tuple_id", "0", "string_id"] -> ["tuple", "index", "string"]
    /// idx: ["tuple_id", "1", "int_id"]    -> ["tuple", "index", "i32"]
    /// idx: ["tuple_id", "2", "bool_id"]   -> ["tuple", "index", "bool"]
    /// ```
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let tuple_id = self.emit_atom("tuple", &format!("tuple[{}]", len));
        Ok(TupleSerializer {
            serializer: self,
            tuple_id,
            index: 0,
        })
    }

    /// ## TUPLE STRUCTS - `idx(container, position, element)`
    /// 
    /// **Decision**: Use positional indexing for tuple structs since they combine
    /// the naming of structs with the positional semantics of tuples.
    /// 
    /// **Covers**: `struct Point(f64, f64)`, `struct Color(u8, u8, u8, u8)`
    /// 
    /// **Relationalization**: Same pattern as tuples but with struct name as type
    /// - Preserves positional access semantics
    /// - Named type for better semantic understanding
    /// - Common for coordinate types, newtype wrappers with multiple fields
    /// 
    /// **Example**: `Point(3.14, 2.71)` becomes:
    /// ```
    /// idx: ["point_id", "0", "x_coord"] -> ["Point", "index", "f64"]
    /// idx: ["point_id", "1", "y_coord"] -> ["Point", "index", "f64"]
    /// ```
    fn serialize_tuple_struct(
        self,
        name: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let struct_id = self.emit_atom("tuple_struct", name);
        Ok(TupleStructSerializer {
            serializer: self,
            struct_id,
            index: 0,
        })
    }

    /// ## ENUM TUPLE VARIANTS - `idx(container, position, element)`
    /// 
    /// **Decision**: Enum variants with tuple-like data use positional indexing
    /// since they represent choice + positional data.
    /// 
    /// **Covers**: `enum Event { Move(f64, f64), Resize(u32, u32) }`
    /// 
    /// **Relationalization**: Positional access within the variant context
    /// - Enum variant acts as container for positional elements
    /// - Preserves both choice semantics (which variant) and position semantics
    /// 
    /// **Example**: `Event::Move(1.0, 2.0)` becomes:
    /// ```
    /// idx: ["move_variant", "0", "x_val"] -> ["enum_variant", "index", "f64"]
    /// idx: ["move_variant", "1", "y_val"] -> ["enum_variant", "index", "f64"]
    /// ```
    fn serialize_tuple_variant(
        self,
        enum_name: &str,
        _variant_index: u32,
        variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let variant_id = self.emit_atom(enum_name, variant);
        Ok(TupleVariantSerializer {
            serializer: self,
            variant_id,
            variant_type: enum_name.to_string(),
            index: 0,
        })
    }

    /// ## KEY-VALUE MAPS - `map_entry(map, key, value)`
    /// 
    /// **Decision**: Use ternary relations for associative containers since the
    /// key-value relationship is the fundamental semantic operation.
    /// 
    /// **Covers**: `HashMap<K,V>`, `BTreeMap<K,V>`, `IndexMap<K,V>`
    /// 
    /// **Relationalization**: `map_entry(container_id, key_id, value_id)`
    /// - Preserves associative lookup semantics: key → value
    /// - Keys and values are full atoms (can be complex types)
    /// - No ordering implied (even for BTreeMap, since iteration order ≠ access semantics)
    /// - Enables CnD layouts focused on key-value relationships
    /// 
    /// **Design Note**: We distinguish this from struct fields because:
    /// - Map keys are data (computed at runtime)
    /// - Struct field names are metadata (known at compile time)
    /// - Maps support dynamic key sets, structs have fixed field sets
    /// 
    /// **Example**: `{"name": "Alice", "age": 30}` becomes:
    /// ```
    /// map_entry: ["map_id", "name_key", "alice_val"] -> ["map", "string", "string"]
    /// map_entry: ["map_id", "age_key", "thirty_val"]  -> ["map", "string", "i32"]
    /// ```
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let map_id = self.emit_atom("map", &format!("map[{}]", len.unwrap_or(0)));
        Ok(MapSerializer {
            serializer: self,
            map_id,
            key_id: None,
        })
    }

    /// ## NAMED STRUCTS - Field name as relation name
    /// 
    /// **Decision**: Use field names directly as relation names because struct fields
    /// represent semantic roles, not positional data or associative lookups.
    /// 
    /// **Covers**: `struct Person { name: String, age: u32 }`
    /// 
    /// **Relationalization**: `field_name(struct_instance, field_value)`
    /// - Each field becomes its own relation type
    /// - Struct type name is used as the atom type (not generic "struct")
    /// - Enables direct semantic querying: "find all names", "find all ages"
    /// - Supports CnD layouts that understand field semantics
    /// 
    /// **Design Rationale**:
    /// - Struct fields are compile-time metadata with semantic meaning
    /// - Field names like "position", "velocity", "color" have domain significance
    /// - Unlike map keys (runtime data) or array indices (positional data)
    /// - Allows field-specific visualization rules in CnD specs
    /// 
    /// **Example**: `Person { name: "Alice", age: 30 }` becomes:
    /// ```
    /// name: ["person_id", "alice_str"] -> ["Person", "string"]
    /// age:  ["person_id", "thirty_int"] -> ["Person", "i32"]
    /// ```
    /// 
    /// This enables CnD rules like:
    /// ```yaml
    /// name: 
    ///   - position: as_label
    /// age:
    ///   - position: bottom_right
    ///   - when: this > 65
    ///     color: gold
    /// ```
    fn serialize_struct(
        self,
        name: &str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let struct_id = self.emit_atom(name, name);  // struct name IS the type
        
        // Collect decorators for this struct type
        self.collect_decorators_for_type(name);
        
        Ok(StructSerializer {
            serializer: self,
            struct_id,
            struct_type: name.to_string(),
        })
    }

    /// ## ENUM STRUCT VARIANTS - Field name as relation name
    /// 
    /// **Decision**: Enum variants with struct-like data use the same field name
    /// pattern as regular structs, since they represent choice + named field data.
    /// 
    /// **Covers**: `enum Shape { Rectangle { width: f64, height: f64 } }`
    /// 
    /// **Relationalization**: Same as structs but with enum_variant as container type
    /// - Preserves both choice semantics (which variant) and field semantics
    /// - Field names retain their semantic meaning within the variant context
    /// 
    /// **Example**: `Shape::Rectangle { width: 10.0, height: 5.0 }` becomes:
    /// ```
    /// width:  ["rect_variant", "ten_val"]  -> ["enum_variant", "f64"]
    /// height: ["rect_variant", "five_val"] -> ["enum_variant", "f64"]
    /// ```
    fn serialize_struct_variant(
        self,
        enum_name: &str,
        _variant_index: u32,
        variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let variant_id = self.emit_atom(enum_name, variant);
        Ok(StructVariantSerializer {
            serializer: self,
            variant_id,
            variant_type: enum_name.to_string(),
        })
    }
}

/// # INDEXABLE SEQUENCE SERIALIZERS
/// 
/// These implement the `idx(container, position, element)` relationalization pattern
/// for collections where position has stable, meaningful semantics.

/// ## Vec<T>, arrays, slices - O(1) indexable collections
/// 
/// **Serialization Pattern**: Each element creates an `idx` relation
/// **Position Encoding**: String representation of 0-based index
/// **Type Preservation**: Element types preserved as individual atoms
pub struct SequenceSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    seq_id: String,
    index: usize,
}

impl<'a> SerializeSeq for SequenceSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let element_id = value.serialize(&mut *self.serializer)?;
        // idx(container, position, element) for O(1) indexable sequences
        self.serializer.push_relation(
            "idx", 
            vec![self.seq_id.clone(), self.index.to_string(), element_id], 
            vec!["sequence", "index", "atom"]
        );
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.seq_id)
    }
}

// Tuples - heterogeneous, fixed positions
pub struct TupleSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    tuple_id: String,
    index: usize,
}

impl<'a> SerializeTuple for TupleSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let element_id = value.serialize(&mut *self.serializer)?;
        // Tuples also use idx - fixed positional semantics
        self.serializer.push_relation(
            "idx", 
            vec![self.tuple_id.clone(), self.index.to_string(), element_id], 
            vec!["tuple", "index", "atom"]
        );
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.tuple_id)
    }
}

// Tuple structs - named but positional
pub struct TupleStructSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    struct_id: String,
    index: usize,
}

impl<'a> SerializeTupleStruct for TupleStructSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let field_id = value.serialize(&mut *self.serializer)?;
        // Tuple structs have positional semantics
        self.serializer.push_relation(
            "idx", 
            vec![self.struct_id.clone(), self.index.to_string(), field_id], 
            vec!["tuple_struct", "index", "atom"]
        );
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.struct_id)
    }
}

pub struct TupleVariantSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    variant_id: String,
    variant_type: String,
    index: usize,
}

impl<'a> SerializeTupleVariant for TupleVariantSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let field_id = value.serialize(&mut *self.serializer)?;
        self.serializer.push_relation(
            "idx", 
            vec![self.variant_id.clone(), self.index.to_string(), field_id], 
            vec![&self.variant_type, "index", "atom"]
        );
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.variant_id)
    }
}

/// # KEY-VALUE MAP SERIALIZERS
/// 
/// These implement the `map_entry(map, key, value)` relationalization pattern
/// for associative containers where key-value relationships are fundamental.

/// ## HashMap, BTreeMap - Associative collections  
/// 
/// **Serialization Pattern**: Each key-value pair creates a `map_entry` relation
/// **Key Handling**: Keys are serialized as full atoms (can be complex types)
/// **Value Handling**: Values are serialized as full atoms (can be complex types)  
/// **Ordering**: No positional semantics - pure associative lookup
pub struct MapSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    map_id: String,
    key_id: Option<String>,
}

impl<'a> SerializeMap for MapSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        self.key_id = Some(key.serialize(&mut *self.serializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let value_id = value.serialize(&mut *self.serializer)?;
        if let Some(key_id) = self.key_id.take() {
            // map_entry(map, key, value) for associative collections
            self.serializer.push_relation(
                "map_entry", 
                vec![self.map_id.clone(), key_id, value_id], 
                vec!["map", "atom", "atom"]
            );
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map_id)
    }
}

/// # STRUCT SERIALIZERS
/// 
/// These implement the field-name-as-relation-name pattern for named data structures
/// where field names carry semantic meaning beyond just data organization.

/// ## Named structs - Semantic field relationships
/// 
/// **Serialization Pattern**: Each field creates a relation named after the field
/// **Type Handling**: Struct name becomes the atom type (not generic "struct")
/// **Field Semantics**: Field names like "position", "velocity" become relation names
/// **Query Benefits**: Enables direct semantic queries like "SELECT * FROM position"
pub struct StructSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    struct_id: String,
    struct_type: String,
}

impl<'a> SerializeStruct for StructSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let field_id = value.serialize(&mut *self.serializer)?;
        // Use field name as relation name: field_name(StructType, value)
        self.serializer.push_relation(
            key, 
            vec![self.struct_id.clone(), field_id], 
            vec![&self.struct_type, "atom"]
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.struct_id)
    }
}

pub struct StructVariantSerializer<'a> {
    serializer: &'a mut JsonDataSerializer,
    variant_id: String,
    variant_type: String,
}

impl<'a> SerializeStructVariant for StructVariantSerializer<'a> {
    type Ok = String;
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let field_id = value.serialize(&mut *self.serializer)?;
        // Enum struct variants also use field names as relations
        self.serializer.push_relation(
            key, 
            vec![self.variant_id.clone(), field_id], 
            vec![&self.variant_type, "atom"]
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.variant_id)
    }
}
