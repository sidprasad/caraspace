use crate::jsondata::*;
use serde::Serialize;
use serde_value::Value;
use std::collections::HashMap;

pub fn export_json_instance<T: Serialize>(value: &T) -> JsonDataInstance {
    let val = serde_value::to_value(value).unwrap();
    let mut state = ExportState::new();
    state.walk_value(&val, None, None);
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

    fn walk_value(&mut self, v: &Value, type_hint: Option<String>, label_hint: Option<String>) -> String {
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
                for item in seq {
                    let child_id = self.walk_value(item, None, None);
                    self.push_relation("item", vec![id.clone(), child_id], vec!["list", "atom"]);
                }
                id
            }
            Value::Map(map) => {
                let id = self.emit_atom("map", label_hint.unwrap_or_else(|| "map".to_string()).as_str());
                for (k, v) in map {
                    let k_id = self.walk_value(k, None, None);
                    let v_id = self.walk_value(v, None, None);
                    self.push_relation("entry", vec![id.clone(), k_id, v_id], vec!["map", "atom", "atom"]);
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
                Some(inner) => self.walk_value(inner, type_hint, label_hint),
                None => self.emit_atom("option", "none"),
            },
            Value::Newtype(inner) => self.walk_value(inner, type_hint, label_hint),
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
}
