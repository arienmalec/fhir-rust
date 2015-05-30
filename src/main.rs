extern crate chrono;
extern crate url;
extern crate rustc_serialize;
mod primitive;
mod element;

use element::{Element,NamedFrom,ElementType};
use rustc_serialize::json::{ToJson};
use primitive::Primitive;


#[allow(dead_code)]
fn main() {

	let e1 = Element::from_name_val("foo",false);
	let e2 = Element::from_name_val("bar",false);
	let e3 = Element::from_name_val("baz",23i32);
	let e_second = Element::from_name_val("second", vec![e3]);
	let e_list = Element::from_name_val("list", vec![
		ElementType::Atom(Primitive::from(true)),
		ElementType::Atom(Primitive::from(true))]);
	let e_top = Element::from_name_val("top", vec![e1,e2,e_second,e_list]);

	let j = e_top.to_json();
	println!("{}",e_top.to_string());
    println!("{}",j.to_string());

}
