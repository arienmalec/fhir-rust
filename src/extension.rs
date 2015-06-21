use std::collections::btree_map::BTreeMap;

use url::Url;
use rustc_serialize::json::{ToJson, Json};

use primitive::Primitive;
use element::{Element,NamedFrom};



pub enum ExtensionValue {
	Atom(Primitive),
	Composite(Element),
	Extensions(Vec<Extension>) 
}

impl ExtensionValue {
	fn value_name(&self) -> String {
		match *self {
			ExtensionValue::Atom(ref p) => p.extension_name(),
			ExtensionValue::Composite(ref e) => e.extension_name(),
			ExtensionValue::Extensions(_) => String::from("extension")
		}
	}

	fn valid_extension(&self) -> bool {
		match *self {
			ExtensionValue::Atom(ref p) => p.valid_extension(),
			ExtensionValue::Composite(ref e) => e.valid_extension(),
			ExtensionValue::Extensions(_) => true
		}
	}
}

trait InternalToJson {
	fn _to_json(&self) -> Json;
}

impl InternalToJson for Element {
	fn _to_json(&self) -> Json {
		self.value.to_json()
	}
}

impl ToJson for ExtensionValue {
	fn to_json(&self) -> Json {
		match *self {
			ExtensionValue::Atom(ref p) => p.to_json(),
			ExtensionValue::Composite(ref e) => e._to_json(),
			ExtensionValue::Extensions(ref v) => v.to_json()
		}
	}
}

pub struct Extension {
	id: Option<String>,
	uri: Url,
	value: ExtensionValue
}

impl ToJson for Extension {
	fn to_json(&self) -> Json {
		let mut o: BTreeMap<String,Json> = BTreeMap::new();
		self.id.as_ref().and_then(|i| o.insert(String::from("id"), Json::String(i.clone())));
		o.insert(String::from("url"),Json::String(self.uri.to_string()));
		o.insert(self.value.value_name(), self.value.to_json());
		Json::Object(o)

	}
}

impl Extension {
	pub fn builder() -> ExtensionBuilder {
		ExtensionBuilder::new()
	}
}


pub struct ExtensionBuilder {
	id: Option<String>,
	uri: Option<Url>,
	value: Option<ExtensionValue>
}

impl ExtensionBuilder {
	pub fn new() -> Self {
		ExtensionBuilder {id: None, uri: None, value: None}
	}

	pub fn id(mut self, id: &str) -> Self {
		self.id = Some(String::from(id));
		self
	}

	pub fn uri(mut self, uri: Url) -> Self {
		self.uri = Some(uri);
		self
	}

	fn set_value(mut self, v: ExtensionValue) -> Self {
		self.value = Some(v);
		self
	}

	fn value(self, v: ExtensionValue) -> Result<Self, &'static str> {
		if self.value.is_some() {return Err("Already has value")}
		if v.valid_extension() {
			Ok(self.set_value(v))
		} else {
			Err("Invalid atomic value")
		}

	}

	pub fn atom(self, p: Primitive) -> Result<Self,&'static str> {
		self.value(ExtensionValue::Atom(p))
	}

	pub fn composite(self, e: Element) -> Result<Self,&'static str> {
		self.value(ExtensionValue::Composite(e))
	}

	pub fn extensions(self, e: Vec<Extension>) -> Result<Self, &'static str> {
		self.value(ExtensionValue::Extensions(e))
	}


	pub fn build(self) -> Result<Extension,&'static str> {
		match (self.uri, self.value) {
			(Some(u), Some(v)) => Ok(Extension{id: self.id, uri: u, value: v}),
			(None, Some(_)) => Err("URI missing"),
			(Some(_),None) => Err("Value missing"),
			(None, None) => Err("Both URI and Value missing")
		}
	}
}

#[test]
fn test_atomic_extension() {
	let e = Extension::builder()
		.uri(Url::parse("http://example.org/is_happy").ok().unwrap())
		.id("ext_id1")
		.atom(Primitive::from(false))
		.and_then(|e| e.build())
		.ok().unwrap();
	let j = Json::from_str(r#"{"url": "http://example.org/is_happy", "id": "ext_id1", "valueBoolean": false}"#).unwrap();
	assert_eq!(j, e.to_json());
}

#[test]
fn test_composite_extension() {
	let elt = Element::with("Coding", vec![
		Element::with("system",Url::parse("http://example.org/mycode").ok().unwrap()),
		Element::with("code","abc123"),
		Element::with("display","Alpha Bravo Charlie 123")]);


	let e = Extension::builder()
		.uri(Url::parse("http://example.org/is_happy").ok().unwrap())
		.composite(elt)
		.and_then(|e| e.build())
		.ok().unwrap();
	let j = Json::from_str(r#"{"url": "http://example.org/is_happy", "valueCoding": {"system": "http://example.org/mycode", "code": "abc123", "display": "Alpha Bravo Charlie 123"}}"#).unwrap();
	assert_eq!(j, e.to_json());
}

#[test]
fn test_extension_with_subextensions() {
	let e1 = Extension::builder()
		.uri(Url::parse("http://example.org/is_happy").ok().unwrap())
		.atom(Primitive::from(false))
		.and_then(|e| e.build())
		.ok().unwrap();
	let e2 = Extension::builder()
		.uri(Url::parse("http://example.org/myalpha").ok().unwrap())
		.atom(Primitive::from("abc"))
		.and_then(|e| e.build())
		.ok().unwrap();
	let e = Extension::builder()
		.uri(Url::parse("http://example.org/happy_alpha").ok().unwrap())
		.extensions(vec![e1, e2])
		.and_then(|e| e.build())
		.ok().unwrap();
	let j = Json::from_str(r#"{"url": "http://example.org/happy_alpha", "extension": [{"url": "http://example.org/is_happy", "valueBoolean": false},{"url": "http://example.org/myalpha", "valueString": "abc"}]}"#).unwrap();
	assert_eq!(j, e.to_json());
}

