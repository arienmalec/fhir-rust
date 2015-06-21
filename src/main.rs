extern crate chrono;
extern crate url;
extern crate rustc_serialize;
mod primitive;
mod element;
mod resource;
mod extension;

use rustc_serialize::json::{ToJson};
use url::Url;

use element::{Element,Value,NamedFrom};
use resource::Resource;
use extension::Extension;
use primitive::Primitive;



#[allow(dead_code)]
fn main() {
	let ext = Extension::builder()
		.uri(Url::parse("http://example.org/is_happy").ok().unwrap())
		.atom(Primitive::from(false))
		.and_then(|e| e.build())
		.ok().unwrap();

	let e1 = Element::with("foo",false);
	let e2 = Element::with("bar",false);
	let e3 = Element::with("baz",23i32);
	let e_second = Element::with("second", vec![e3]);
	let e_list = Element::with("list", vec![
		Value::from(true),
		Value::from(true)]);
	let e_top = Resource::new("top")
		.add_elt(e1)
		.add_elt(e2)
		.add_elt(e_second)
		.add_elt(e_list)
		.add_ext(ext);

	let j = e_top.to_json();
    println!("{}",j.to_string());

}
