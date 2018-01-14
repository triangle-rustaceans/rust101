#[macro_use] extern crate serde_derive;
extern crate clap;
extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::fs::File;

pub mod cyoa {
    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CYOA {
        pub title: String,
        pub author: String,
        pub locations: Vec<CYOAPage>,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CYOAPage {
        pub id: u64,
        pub text: String,
        pub actions: Option<Vec<CYOAAction>>,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CYOAAction {
        pub text: String,
        pub label: String,
        pub target: u64,
    }
}

fn main() {
    let reader = File::open("../data/adventure.json").expect("data file");
    let res: Result<cyoa::CYOA, _> = serde_json::from_reader(reader);
    match res {
        Ok(value) => println!("{} {}.", value.title, value.author),
        Err(error) => println!("An Error, {:?}", error),
    };
}
