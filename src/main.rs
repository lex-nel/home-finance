extern crate rusqlite;
extern crate time;
extern crate home_finance;

use rusqlite::Connection;
use std::io;
use time::Timespec;
use home_finance::init::create;
// use std::env;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

fn get_user() {
    let conn = Connection::open("./home_finance.sqlite3").unwrap();

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        &[],
    ).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        &[&me.name, &me.time_created, &me.data],
    ).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person")
        .unwrap();
    let person_iter = stmt.query_map(&[], |row| Person {
        id: row.get(0),
        name: row.get(1),
        time_created: row.get(2),
        data: row.get(3),
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let arg: &str = &args[1];

    // match arg {
    //     "help" => println!("This is help"),
    //     "info" => println!("This is info"),
    //     _ => println!("something else"),
    // }

    println!("Guess the number!");

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim() {
            "user" => get_user(),
            "init" => create::tables(),
            "exit" => {
                println!("Good buy");
                break;
            }
            _ => println!("something else"),
        }
    }
}
