use crate::jsondata::*;
use serde::Serialize;
use serde_value::Value;
use std::collections::HashMap;

pub fn export_json_instance<T: Serialize>(value: &T) -> JsonDataInstance {
    let val = serde_value::to_value(value).unwrap();
    let mut state = ExportState::new();
    state.walk_value(&val, None, None, true);
    JsonDataInstance {
        atoms: state.atoms,
        relations: state.relations.into_values().collect(),
    }
}

struct ExportState {
    counter: usize,
    atoms: Vec<IAtom>,
    relations: HashMap<String, IRelation>,
}

impl ExportState {
    fn new() -> Self {
        Self {
            counter: 0,
            atoms: vec![],
            relations: HashMap::new(),
        }
    }

    fn fresh_id(&mut self) -> String {
        let id = format!("atom{}", self.counter);
        self.counter += 1;
        id
    }

    fn walk_value(&mut self, v: &Value, type_hint: Option<String>, label_hint: Option<String>, _is_root: bool) -> String {
        match v {
            Value::Bool(b) => self.emit_atom("bool", &b.to_string()),
            Value::I64(i) => self.emit_atom("int", &i.to_string()),
            Value::U64(u) => self.emit_atom("int", &u.to_string()),
            Value::F64(f) => self.emit_atom("float", &f.to_string()),
            Value::Char(c) => self.emit_atom("char", &c.to_string()),
            Value::String(s) => self.emit_atom("string", s),
            Value::Unit => self.emit_atom("unit", "()"),
            Value::Seq(seq) => {
                let id = self.emit_atom("list", "list");
                for (index, item) in seq.iter().enumerate() {
                    let child_id = self.walk_value(item, None, None, false);
                    let index_id = self.emit_atom("int", &index.to_string());
                    // Create ternary relation: list -> index -> item
                    self.push_relation("items", vec![id.clone(), index_id, child_id], vec!["list", "int", "atom"]);
                }
                id
            }
            Value::Map(map) => {
                // Detect if this looks like a struct vs a generic map using heuristics
                let is_struct = self.looks_like_struct(map);
                
                let (atom_type, relation_name) = if is_struct {
                    ("struct", "field")
                } else {
                    ("map", "entry")
                };
                
                let label = if is_struct {
                    label_hint.unwrap_or_else(|| "struct".to_string())
                } else {
                    label_hint.unwrap_or_else(|| "map".to_string())
                };
                
                let id = self.emit_atom(atom_type, &label);
                for (k, v) in map {
                    let k_id = self.walk_value(k, None, None, false);
                    let v_id = self.walk_value(v, None, None, false);
                    self.push_relation(relation_name, vec![id.clone(), k_id, v_id], vec![atom_type, "atom", "atom"]);
                }
                id
            }
            Value::U8(u) => self.emit_atom("u8", &u.to_string()),
            Value::U16(u) => self.emit_atom("u16", &u.to_string()),
            Value::U32(u) => self.emit_atom("u32", &u.to_string()),
            Value::I8(i) => self.emit_atom("i8", &i.to_string()),
            Value::I16(i) => self.emit_atom("i16", &i.to_string()),
            Value::I32(i) => self.emit_atom("i32", &i.to_string()),
            Value::F32(f) => self.emit_atom("float", &f.to_string()),
            Value::Option(opt) => match opt {
                Some(inner) => self.walk_value(inner, type_hint, label_hint, false),
                None => self.emit_atom("option", "none"),
            },
            Value::Newtype(inner) => self.walk_value(inner, type_hint, label_hint, false),
            Value::Bytes(bytes) => self.emit_atom("bytes", &format!("{:?}", bytes)),
        }
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

    /// Heuristic to determine if a map looks like a struct rather than a generic map.
    /// We consider it a struct if all keys are strings that look like field names.
    fn looks_like_struct(&self, map: &std::collections::BTreeMap<Value, Value>) -> bool {
        if map.is_empty() {
            return false;
        }

        // Check if all keys are strings that look like field names
        for key in map.keys() {
            match key {
                Value::String(s) => {
                    // Check if it looks like a field name (simple heuristic)
                    // Field names should start with letter/underscore and contain only alphanumeric/underscore
                    if s.is_empty() {
                        return false;
                    }
                    
                    let first_char = s.chars().next().unwrap();
                    if !first_char.is_alphabetic() && first_char != '_' {
                        return false;
                    }
                    
                    if !s.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        return false;
                    }
                }
                _ => return false, // Non-string key suggests generic map
            }
        }
        
        true
    }
}
