use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() {

    let mut arguments = std::env::args().skip(1);
    let command = arguments.next().unwrap();
    if !Path::new("kv.db").exists() {

        println!("File kv.db does not exist");
        std::fs::File::create("kv.db").unwrap();
    }

    match &command[..] {
        "add" => {
            let key = arguments.next().unwrap();
            let value = arguments.next().unwrap();
            
            let mut db = Database::new().expect("Creating DB failed");
            let _ = db.insert(&key, &value);
        },
        "get" => {
            let key = arguments.next().unwrap();

            let db = Database::new().expect("Creating DB failed");
            let value = match db.read(&key) {
                None => None,
                Some(val) => Some(val.to_owned())
            };
            if value != None {
                println!("{}", value.as_deref().unwrap());
            }
        },
        "delete" => {
            let key = arguments.next().unwrap();

            let mut db = Database::new().expect("Creating DB failed");
            match db.delete(&key) {
                None => println!("Key does not exists"),
                Some(_) => println!("Done")
            };
            let _ = do_flush(db);
        },
        _ => println!("Option not allowed")
    }
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {

        let mut db_map = HashMap::new();
        let file_content = std::fs::read_to_string("kv.db")?;

        for line in file_content.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key found");
            let value = chunks.next().expect("No value found");

            db_map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database{ map: db_map })
    }

    fn insert(&mut self, key: &String, value: &String) -> std::io::Result<()> {
        
        self.map.insert(key.to_owned(), value.to_owned());

        let content = format!("{}\t{}\n", key.to_owned(), value.to_owned());
        let mut file = OpenOptions::new().write(true).append(true).open("kv.db").unwrap();
        file.write_all(content.as_bytes())
    }

    fn read(&self, key: &String) -> Option<&String> {
        
        self.map.get(key)
    }

    fn delete(&mut self, key: &String) -> Option<String> {

        self.map.remove(key)
    }
}

fn do_flush(database: Database) -> std::io::Result<()> {

    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(&value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}