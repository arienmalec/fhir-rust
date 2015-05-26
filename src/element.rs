use std::fmt;
use std::iter::repeat;
use std::convert::{From};
use std::collections::btree_map::BTreeMap;
use rustc_serialize::json::{self, ToJson, Json, Array};
use primitive::Primitive;


pub enum ElementType {
	Atom(Primitive),
	List(Vec<ElementType>),
	Elt(Vec<Element>)
}

impl ToJson for ElementType {
	fn to_json(&self) -> Json {
		match *self {
			ElementType::Atom(ref v) => v.to_json(),
			ElementType::List(ref v) => {
				Json::Array(v.iter().map(|e| e.to_json()).collect::<Vec<Json>>())
			},
			ElementType::Elt(ref v) => {
				let mut o: BTreeMap<String,Json> = BTreeMap::new();
				for e in v.iter() {
					o.insert(e.name.clone(), e.value.to_json());
				}
				Json::Object(o)
			}
		}
	}
}


impl From<bool> for ElementType {
	fn from(b:bool) -> Self {
		ElementType::Atom(Primitive::from(b))
	}
}

impl From<i32> for ElementType {
	fn from(v: i32) -> Self {
		ElementType::Atom(Primitive::Int(v))
	}
}

impl From<Vec<Element>> for ElementType {
	fn from(v: Vec<Element>) -> Self {
		ElementType::Elt(v)
	}
}


pub struct Element {
	pub name: String,
	pub value: ElementType
	//pub id: Option<String>,
	//pub extensions: Option<Vec<Extension>>
}

pub trait NamedFrom<T> {
	fn from_name_val(name: &str, val: T) -> Self;
}

impl NamedFrom<bool> for Element {
	fn from_name_val(name: &str, val: bool) -> Self {
		Element {
			name: name.to_string(),
			value: ElementType::from(val)
		}
	}
}

impl NamedFrom<i32> for Element {
	fn from_name_val(name: &str, val: i32) -> Self {
		Element {
			name: name.to_string(),
			value: ElementType::from(val)
		}
	}
}

impl NamedFrom<Vec<Element>> for Element {
	fn from_name_val(name: &str, val: Vec<Element>) -> Self {
		Element {
			name: name.to_string(),
			value: ElementType::from(val)
		}
	}
}


impl Element {
	pub fn to_string(&self) -> String {
		self.recursive_to_string(0)
	}

	fn recursive_to_string(&self,level: usize) -> String {

		fn value_to_string(v: &ElementType, level: usize) -> String {
			fn recursive_elt_vec_to_string (v: &Vec<Element>, level: usize) -> String {
				v.iter().map(|e| e.recursive_to_string(level)).collect::<Vec<String>>().connect("\n")
			}
			fn value_list_to_string(v: &Vec<ElementType>, level: usize) -> String {
				v.iter().map(|e| value_to_string(&e,level)).collect::<Vec<String>>().connect(",")
			}
			match *v {
	 			ElementType::Atom(ref v) => format!("{}",v),
 				ElementType::List(ref v) => format!("[{}]", value_list_to_string(v, level)),
 				ElementType::Elt(ref v) => format!("\n{}",recursive_elt_vec_to_string(v, level + 1))			
			}
		}

		let spaces: String = repeat("  ").take(level).collect::<Vec<&str>>().concat();
		let label = format!("{}: ",self.name);
		let value = value_to_string(&self.value, level);

		format!("{}{}{}", spaces, label, value)
	}

}


impl fmt::Display for Element {
 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 		write!(f,"{}",self.to_string())
 	}
}

impl ToJson for Element {
	// Will only be valid JSON if self.value is an element list.
	fn to_json(&self) -> Json {
		self.value.to_json()
	}
}

#[test]
fn test_simple_primitive_elt() {
	let e = Element {
		name: "foo".to_string(),
		value: ElementType::from(false)
	};
	assert_eq!("foo: false", format!("{}", e));
}

fn make_test_elt() -> Element {
	let e1 = Element::from_name_val("foo",false);
	let e2 = Element::from_name_val("bar",false);
	let e3 = Element::from_name_val("baz",23i32);
	let e_second = Element::from_name_val("second", vec![e3]);
	let e_top = Element::from_name_val("top", vec![e1,e2,e_second]);
	e_top

}

#[test]
fn test_compound_elt() {

	let expected = "top: 
  foo: false
  bar: false
  second: 
    baz: 23";
  assert_eq!(expected, make_test_elt().to_string());

}