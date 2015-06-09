extern crate chrono;
extern crate url;
extern crate rustc_serialize;
mod primitive;
mod element;
mod resource;
mod new_element;

use new_element::{Element,Value,NamedFrom};
use rustc_serialize::json::{ToJson};
use resource::Resource;


#[allow(dead_code)]
fn main() {

	let e1 = Element::with("foo",false);
	let e2 = Element::with("bar",false);
	let e3 = Element::with("baz",23i32);
	let e_second = Element::with("second", vec![e3]);
	let e_list = Element::with("list", vec![
		Value::from(true),
		Value::from(true)]);
	let e_top = Resource::new("top", vec![e1,e2,e_second,e_list]);

	let j = e_top.to_json();
    println!("{}",j.to_string());

}
