use std::convert::{From};
use std::collections::btree_map::BTreeMap;

use rustc_serialize::json::{ToJson, Json};
use url::{Url};
use chrono::{DateTime,FixedOffset};

use primitive::{Primitive, Dec, Time};

pub struct Element {
	name: String,
	value: Value
}

impl InternalToJson for Element {
	fn _to_json(&self) -> Json {
		self.value.to_json()
	}
}


pub trait NamedFrom<T> {
	fn from_name_val(name: &str, val: T) -> Self;
}

macro_rules! gen_named {
	($t:ty) => {
		impl NamedFrom<$t> for Element {
			fn from_name_val(name: &str, v: $t) -> Self {
				Element {
					name: name.to_string(),
					value: Value::from(v)
				}
			}
		}
	}
}

gen_named!(bool);
gen_named!(i32);
gen_named!(u32);
gen_named!(Dec);
gen_named!(String);
gen_named!(Time);
gen_named!(Url);
gen_named!(DateTime<FixedOffset>);

impl NamedFrom<Vec<Element>> for Element {
	fn from_name_val(name: &str, val: Vec<Element>) -> Self {
		Element {
			name: name.to_string(),
			value: Value { value: ValueType::Elt(val), id: None}
		}
	}
}

impl NamedFrom<Vec<Value>> for Element {
	fn from_name_val(name: &str, val: Vec<Value>) -> Self {
		Element {
			name: name.to_string(),
			value: Value { value: ValueType::List(val), id: None}
		}
	}
}


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
	value: ValueType,
	id: Option<String>
}

impl Value {

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
}

trait InternalToJson {
	fn _to_json(&self) -> Json;
}

impl InternalToJson for Vec<Element> {
	fn _to_json(&self) -> Json {
		let mut o: BTreeMap<String,Json> = BTreeMap::new();
		for e in self.iter() {
			o.insert(e.name.clone(), e.value.to_json());
			e.value.id_ext_to_json()
				.and_then(|j| o.insert(format!("_{}",e.name),j));
		}
		Json::Object(o)
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

fn make_test_elt() -> Element {
	let mut e1 = Element::from_name_val("foo",false);
	e1.value.id = Some("quux".to_string());
	let e1 = e1;
	let e2 = Element::from_name_val("bar",false);
	let e3 = Element::from_name_val("baz",23u32);
	let e_second = Element::from_name_val("second", vec![e3]);
	let e_list = Element::from_name_val("list", vec![
		Value::from(true),
		Value {value: ValueType::Atom(Primitive::from(true)), id: Some("abc123".to_string())}]);
	let e_top = Element::from_name_val("top", vec![e1,e2,e_second,e_list]);
	e_top
}

#[test]
fn test_compound_elt() {
	let expected = Json::from_str(r#"{"foo": false, "_foo": {"id": "quux"}, "bar": false, "second": { "baz": 23 }, "list": [true,true], "_list": [null, {"id":"abc123"}]}"#).unwrap();
  	assert_eq!(expected, make_test_elt()._to_json());
}