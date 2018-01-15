#[macro_use] extern crate serde_derive;
extern crate clap;
extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

pub mod cyoa {
    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CYOA {
        pub title: String,
        pub author: String,
        pub locations: Vec<CYOALocation>,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CYOALocation {
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

fn page(cyoa: &cyoa::CYOA, index: u64) -> Option<u64> {
    let location = cyoa.locations.get((index) as usize)?;
    println!("\n{}", &location.text);
    let mut i = 1;
    let action_vec = match &location.actions {
        &Some(ref action_vec) => action_vec,
        &None => return None,
    };
    println!();
    for action in action_vec {
        println!("{}) {} ({})", i, action.text, action.label);
        i += 1;
    }
    let mut choice = String::with_capacity(64);
    let mut next_action = None;
    while next_action.is_none() {
        choice.truncate(0);
        stdin().read_line(&mut choice).expect("reading input");
        let choice = choice.trim();
        next_action = match choice.parse::<u64>() {
            Ok(n) => action_vec.get(n as usize - 1),
            Err(_) => action_vec.iter().find(|action| action.label == choice),
        };
    }
    println!("{:?}", next_action.unwrap().text);
    next_action.map(|action| action.target)
}

fn begin(cyoa: &cyoa::CYOA) {
    let mut next = page(cyoa, 0);
    while next.is_some() {
        next = page(cyoa, next.unwrap());
    }
    println!("The end.");
}

fn main() {
    let mut reader = File::open("../data/adventure.json").expect("data file");
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("read file");
    let res: Result<cyoa::CYOA, _> = serde_json::from_str(&s);
    match res {
        Ok(value) => {
            println!("{}, by {}.", value.title, value.author);
            begin(&value);
        },
        Err(error) => {
            println!("An error occurred: {:?}", error);
            ::std::process::exit(1);
        },
    };
}
