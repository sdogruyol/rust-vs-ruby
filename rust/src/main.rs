extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::collections::BTreeMap;
use nickel::{Nickel,HttpRouter};
use rustc_serialize::json::{Json, ToJson};

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
          let person = Person{
            first_name: "Serdar".to_string(),
            last_name: "Dogruyol".to_string()
          };
          person.to_json()
        }
    });

    server.listen("127.0.0.1:9292");
}

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
  first_name: String,
  last_name: String,
}

impl ToJson for Person {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("first_name".to_string(), self.first_name.to_json());
        map.insert("last_name".to_string(), self.last_name.to_json());
        Json::Object(map)
    }
}
