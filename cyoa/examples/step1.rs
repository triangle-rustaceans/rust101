// extern crate serde;
extern crate serde_json;

use serde_json::{Value, from_str};

fn examine_json() -> Option<Value> {
    let val: Value = from_str(r#"
        {"name": "Triangle Rustaceans",
         "dates": ["Nov 25, 2017", "Jan 22, 2018", "20180227"],
         "welcoming": true
        }
    "#).ok()?;
    /*
    let val: Value = match val {
        Ok(val) => val,
        Err(err) => {
            println!("Could not parse JSON {:?}", err);
            return;
        }
    };
    */
    println!("{}", val["name"]);
    println!("{}", val["dates"]);
    println!("{}", val["dates"][0]);
    println!("{}", val["dates"][2]);
    for date in val["dates"].as_array()? {
        println!("{}", date.as_str()?);
    }
    Some(val)
}

fn main() {
    println!("{:?}", examine_json());
}
