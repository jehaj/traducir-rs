use sqlite;
use std::fs::remove_file;
use std::path::Path;
use scraper::{Html, Selector};
use sqlite::State;

struct Entry {
    english: String,
    danish: String,
    source: String,
    active: i32
}

fn main() {
    let path_to_db = Path::new(get_database_name());
    let db_existed = path_to_db.exists();
    print!("Opening connection to database... ");
    let connection = sqlite::open(get_database_name()).unwrap();
    println!("done.");
    if !db_existed {
        println!("Creating schemas and filling db with data.");
        // create schemas
        connection.execute(get_schema_begreber()).expect("Failed to create table.");
        // tell to download raw data if not exists (use download.sh) and exit
        // scrape downloaded data
        let mut entries = vec![];
        entries.append(&mut get_entries_from_klid());
        entries.append(&mut get_entries_from_sdu());
        entries.append(&mut get_entries_from_topdatamat());
        // insert entries into db
        connection.execute("BEGIN TRANSACTION;").unwrap();
        let query = "INSERT INTO Begreber (EngelskUdgave, DanskUdgave, Kilde) VALUES (?, ?, ?)";
        let mut statement = connection.prepare(query).expect("Prepared statement.");
        for entry in entries {
            statement.reset().expect("Failed to reset statement");
            statement.bind((1, entry.english.as_str())).expect("Failed binding first parameter");
            statement.bind((2, entry.danish.as_str())).expect("Failed binding second parameter");
            statement.bind((3, entry.source.as_str())).expect("Failed binding third parameter");
            statement.next().expect("Failed to execute prepared statement.");
        }
        connection.execute("COMMIT;").unwrap();
    }
    //delete_database();
    println!("See you next time :)");
}

fn delete_database() {
    match remove_file(get_database_name()) {
        Ok(_) => println!("Successfully deleted database."),
        Err(_) => println!("An error happened while deleting database.")
    };
}

fn get_database_name() -> &'static str {
    "db.db"
}

fn get_schema_begreber() -> String {
    String::from("CREATE TABLE \"Begreber\" (
	\"EngelskUdgave\"	TEXT NOT NULL,
	\"DanskUdgave\"	TEXT NOT NULL,
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
    let mut entries = vec![];
    let html = std::fs::read_to_string("raw_data/klid.html")
        .expect("Failed to read klid.html");

    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse("body > pre > b").unwrap();

    for element in document.select(&selector) {
        let alphabet_text = element.next_sibling().expect("yes").value().as_text().unwrap().trim();
        let alphabet_text = alphabet_text.replace('\t', "        ");
        let texts = alphabet_text.split('\n');
        for text in texts {
            // remove bad entries: apparently all of which are below 32 in length
            if text.len() < 32 { continue; }
            let key = text[0..32].trim();
            let value = text[32..].trim();
            // add entry
            let entry = Entry {
                english: key.to_string(),
                danish: value.to_string(),
                source: "klid.dk".to_string(),
                active: 1,
            };
            entries.push(entry);
        }
    }
    entries
}

fn get_entries_from_sdu() -> Vec<Entry> {
    let mut entries = vec![];
    entries
}

fn get_entries_from_topdatamat() -> Vec<Entry> {
    let mut entries = vec![];
    entries

}
