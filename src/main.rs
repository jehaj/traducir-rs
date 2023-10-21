use sqlite;
use std::fs::remove_file;
use std::path::Path;

struct Entry {
    english: String,
    danish: String,
    description: String,
    source: String,
    active: i32
}

fn main() {
    let path_to_db = Path::new(get_database_name().as_str());
    let db_existed = path_to_db.exists();
    print!("Opening connection to database... ");
    let connection = sqlite::open(get_database_name()).unwrap();
    println!("done.");
    if !db_existed {
        println!("Creating schemas and filling db with data.");
        // create schemas
        // tell to download raw data if not exists (use download.sh) and exit
        // scrape downloaded data
        let mut entries = vec![];
        entries.append(&mut get_entries_from_klid());
        entries.append(&mut get_entries_from_sdu());
        entries.append(&mut get_entries_from_topdatamat());
        // insert entries into db
        let query = "INSERT INTO Begreber (EngelskUdgave, DanskUdgave, Kilde) VALUES (?, ?, ?)";
        let mut statement = connection.prepare(query).expect("Prepared statement.");
        for entry in entries {
            statement.reset().expect("Failed to reset statement");
            statement.bind((1, entry.english.as_str())).expect("Failed binding first parameter");
            statement.bind((2, entry.danish.as_str())).expect("Failed binding second parameter");
            statement.bind((3, entry.source.as_str())).expect("Failed binding third parameter");
            statement.next().expect("Failed to execute prepared statement.");
        }
    }
    delete_database();
    println!("See you next time :)");
}

fn delete_database() {
    match remove_file(get_database_name()) {
        Ok(_) => println!("Successfully deleted database."),
        Err(_) => println!("An error happened while deleting database.")
    };
}

fn get_database_name() -> String {
    String::from("db.db")
}

fn get_schema_begreber() -> String {
    String::from("CREATE TABLE \"Begreber\" (
	\"EngelskUdgave\"	TEXT NOT NULL,
	\"DanskUdgave\"	TEXT NOT NULL,
	\"Beskrivelse\"	TEXT,
	\"Kilde\"         TEXT,
	\"Aktiv\"         INTEGER DEFAULT 1,
	\"Id\"	INTEGER,
	PRIMARY KEY(\"Id\" AUTOINCREMENT)
);")
}

fn get_schema_index() -> String {
    String::from("CREATE VIRTUAL TABLE fts USING fts5(
    EngelskUdgave,
    DanskUdgave,
    Kilde UNINDEXED,
    Id UNINDEXED,
    content=Begreber,
    content_rowid=Id
);")
}

fn get_data_query() -> String {
    String::from("INSERT INTO fts SELECT EngelskUdgave, DanskUdgave, Kilde, Id FROM Begreber;")
}

fn get_optimize_query() -> String {
    String::from("INSERT INTO fts(fts) VALUES('optimize');")
}

fn get_entries_from_klid() -> Vec<Entry> {

}


fn get_entries_from_sdu() -> Vec<Entry> {

}

fn get_entries_from_topdatamat() -> Vec<Entry> {

}
