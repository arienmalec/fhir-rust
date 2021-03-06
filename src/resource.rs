use std::collections::btree_map::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use url::Url;


use element::{Element,NamedFrom};
use extension::Extension;
use primitive::Primitive;

pub struct Resource {
	pub name: String,
	pub extensions: Vec<Extension>,
	pub elts: Vec<Element>
}

impl Resource {
	pub fn new(name: &str) -> Self {
		Resource {name:String::from(name), elts: Vec::new(), extensions: Vec::new()}
	}

	pub fn new_with_elts(name: &str, elts: Vec<Element>) -> Self {
		let mut r = Self::new(name);
		r.elts = elts;
		r
	}

	pub fn add_elt(mut self, e: Element) -> Self {
		self.elts.push(e);
		self
	}

	pub fn add_ext(mut self, e: Extension) -> Self {
		self.extensions.push(e);
		self
	}

	pub fn has_extensions(&self) -> bool {
		self.extensions.len() > 0
	}
}

impl ToJson for Resource {
	fn to_json(&self) -> Json {
		let mut o: BTreeMap<String,Json> = BTreeMap::new();
		o.insert("resourceType".to_string(),Json::String(self.name.clone()));
		for e in self.elts.iter() {
			o.insert(e.name.clone(), e.value.to_json());
		}
		if self.has_extensions() {
			o.insert(String::from("extension"), self.extensions.to_json());
		}
		Json::Object(o)
	}
}

#[test]
fn test_resource_to_json () {
	let r = Resource::new("foo")
		.add_elt(Element::with("bar",false))
		.add_elt(Element::with("baz",true));
	let r2 = Resource::new_with_elts("foo", vec![
		Element::with("bar",false),
		Element::with("baz",true)]);

	let j = Json::from_str("{\"resourceType\": \"foo\",\"bar\": false,\"baz\": true}").unwrap();

	assert_eq!(j, r.to_json());
	assert_eq!(j, r2.to_json());
}

#[test]
fn test_resource_with_ext () {
	let e = Extension::builder()
		.uri(Url::parse("http://example.org/is_happy").ok().unwrap())
		.atom(Primitive::from(false))
		.and_then(|e| e.build())
		.ok().unwrap();
	let r = Resource::new("foo")
		.add_elt(Element::with("bar",false))
		.add_elt(Element::with("baz",true))
		.add_ext(e);

	let j = Json::from_str(r#"{"resourceType": "foo", "extension": [{"url": "http://example.org/is_happy", "valueBoolean": false}], "bar": false, "baz": true}"#).unwrap();

	assert_eq!(j, r.to_json());
}