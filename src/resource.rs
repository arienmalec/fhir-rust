use std::collections::btree_map::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

use element::{Element,NamedFrom};

pub struct Resource {
	pub name: String,
	pub elts: Vec<Element>
}

impl Resource {
	pub fn new(name: &str, elts: Vec<Element>) -> Self {
		Resource {name: String::from(name), elts: elts}
	}
}

impl ToJson for Resource {
	fn to_json(&self) -> Json {
		let mut o: BTreeMap<String,Json> = BTreeMap::new();
		o.insert("resourceType".to_string(),Json::String(self.name.clone()));
		for e in self.elts.iter() {
			o.insert(e.name.clone(), e.value.to_json());
		}
		Json::Object(o)
	}
}

#[test]
fn test_resource_to_json () {
	let r = Resource::new("foo", vec![Element::with("bar",false),
			Element::with("baz",true)]);

	assert_eq!(Json::from_str("{\"resourceType\": \"foo\",\"bar\": false,\"baz\": true}").unwrap(),
		r.to_json());
}