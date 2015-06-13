use std::collections::btree_map::BTreeMap;

use rustc_serialize::json::{ToJson, Json};
use url::{Url};
use chrono::{DateTime,FixedOffset};

use element::{Element,InternalToJson};
use primitive::{Primitive, Dec, Time};



pub enum ValueType {
	Atom(Primitive),
	List(Vec<Value>),
	Elt(Vec<Element>)
}

impl ToJson for ValueType {
	fn to_json(&self) -> Json {
		match *self {
			ValueType::Atom(ref v) => v.to_json(),
			ValueType::List(ref v) => v.to_json(),
			ValueType::Elt(ref v) => v._to_json()
		}
	}
}


pub struct Value {
	pub value: ValueType,
	pub id: Option<String>
}

impl Value {

	pub fn keys(&self, name: &str) -> Vec<(String,Json)>{
		let mut v = Vec::new();
		v.push((String::from(name),self.to_json()));
		self.id_ext_to_json()
			.map(|j| v.push((format!("_{}",name),j)));
		v
	}

	fn make_idext_ojb(id: &String) -> Json {
		let mut o: BTreeMap<String,Json> = BTreeMap::new();
		o.insert("id".to_string(),Json::String(id.to_string()));
		Json::Object(o)
	}

	fn list_idext_to_json(list: &Vec<Value>) -> Option<Json> {
		let mut found = false;
		let mut retlist: Vec<Json> = Vec::new();
		for v in list {
			if let Some(ref i) = v.id {
				found = true;
				retlist.push(Value::make_idext_ojb(i));				
			} else {
				retlist.push(Json::Null);
			}
		}
		if found { Some(Json::Array(retlist)) } else { None }
	}

	fn simple_idext_to_json(id: Option<&String>) -> Option<Json> {
		id.map(Value::make_idext_ojb)
	}


	fn id_ext_to_json(&self) -> Option<Json> {
		if let ValueType::List(ref v) = self.value {
			Value::list_idext_to_json(v)
		} else {
			Value::simple_idext_to_json(self.id.as_ref())
		}
	}

	pub fn id(mut self, id: &str) -> Self {
		self.id = Some(String::from(id));
		self
	}

}

macro_rules! gen_from {
	($t:ty) => {
		impl From<$t> for Value {
			fn from(v: $t) -> Self {
				Value {
					value: ValueType::Atom(Primitive::from(v)),
					id: None
				}
			}
		}
	}
}

gen_from!(bool);
gen_from!(i32);
gen_from!(u32);
gen_from!(Dec);
gen_from!(String);
gen_from!(Time);
gen_from!(Url);
gen_from!(DateTime<FixedOffset>);

impl From<Vec<Element>> for Value {
	fn from(v: Vec<Element>) -> Self {
		Value {
			value: ValueType::Elt(v),
			id: None
		}
	}
}


impl ToJson for Value {
	fn to_json(&self) -> Json {
		self.value.to_json()
	}
}

#[test]
fn test_bool_value() {
	let v = Value::from(false);
	assert_eq!(Json::Boolean(false), v.to_json());
}
