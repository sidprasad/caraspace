use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct JsonDataInstance {
    pub atoms: Vec<IAtom>,
    pub relations: Vec<IRelation>,
}

#[derive(Serialize, Debug)]
pub struct IAtom {
    pub id: String,
    pub r#type: String,
    pub label: String,
}

#[derive(Serialize, Debug)]
pub struct ITuple {
    pub atoms: Vec<String>,
    pub types: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct IRelation {
    pub id: String,
    pub name: String,
    pub types: Vec<String>,
    pub tuples: Vec<ITuple>,
}
