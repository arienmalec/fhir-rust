use std::convert::{From};
use std::collections::btree_map::BTreeMap;

use rustc_serialize::json::{ToJson, Json};
use url::{Url};
use chrono::{DateTime,FixedOffset};

use primitive::{Dec, Time};

pub mod value;
pub use element::value::{Value,ValueType};

pub struct Element {
	pub name: String,
	pub value: Value
}

impl Element {
	pub fn id(mut self, id: &str) -> Self {
		let v = self.value;
		self.value = v.id(id);
		self
	}
}

trait InternalToJson {
	fn _to_json(&self) -> Json;
}

impl InternalToJson for Vec<Element> {
	fn _to_json(&self) -> Json {
		let mut o: BTreeMap<String,Json> = BTreeMap::new();
		for e in self.iter() {
			let mut keys = e.value.keys(&e.name);
			while let Some((name, json)) = keys.pop()  {
				o.insert(name,json);
			}
		}
		Json::Object(o)
	}
}

impl InternalToJson for Element {
	fn _to_json(&self) -> Json {
		self.value.to_json()
	}
}


pub trait NamedFrom<T> {
	fn with(name: &str, val: T) -> Self;
}

macro_rules! gen_named {
	($t:ty) => {
		impl NamedFrom<$t> for Element {
			fn with(name: &str, v: $t) -> Self {
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
	fn with(name: &str, val: Vec<Element>) -> Self {
		Element {
			name: name.to_string(),
			value: Value { value: ValueType::Elt(val), id: None}
		}
	}
}

impl NamedFrom<Vec<Value>> for Element {
	fn with(name: &str, val: Vec<Value>) -> Self {
		Element {
			name: name.to_string(),
			value: Value { value: ValueType::List(val), id: None}
		}
	}
}


fn make_test_elt() -> Element {
	let e1 = Element::with("foo",false)
				.id("quux");
	let e2 = Element::with("bar",false);
	let e3 = Element::with("baz",23u32);
	let e_second = Element::with("second", vec![e3]);
	let e_list = Element::with("list", vec![
		Value::from(true),
		Value::from(true).id("abc123")]);
	let e_top = Element::with("top", vec![e1,e2,e_second,e_list]);
	e_top
}

#[test]
fn test_compound_elt() {
	let expected = Json::from_str(r#"{"foo": false, "_foo": {"id": "quux"}, "bar": false, "second": { "baz": 23 }, "list": [true,true], "_list": [null, {"id":"abc123"}]}"#).unwrap();
  	assert_eq!(expected, make_test_elt()._to_json());
}