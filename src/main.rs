use sqlite;
use std::fs::remove_file;

fn main() {
    print!("Opening connection to database... ");
    let connection = sqlite::open(get_database_name()).unwrap();
    println!("done.");
    delete_database();
    println!("See you next time :)");
}

fn delete_database() {
    match remove_file(get_database_name()) {
        Ok(_) => println!("Succesfully deleted database."),
        Err(_) => println!("An error happened while deleting database.")
    };
}

fn get_database_name() -> String {
    String::from("db.db")
}
