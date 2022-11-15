use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


fn main() -> Result<()> {
    // in memory database is stored in RAM and a file for the DB will not be created
    // use open() and path to a file to create a db file if you want persistent db
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BLOB
        )",
        [], // with rusqlite version "0.24.1" you have to use NO_PARAMS here instead. 
        // The rusqlite version here is "0.27.0"
    )?;

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };

    let me1 = Person {
        id: 1,
        name: "Steven1".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person(name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    conn.execute(
        "INSERT INTO person(name, data) VALUES (?1, ?2)",
        params![me1.name, me1.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;

    let person_iter = stmt.query_map([], |row| {
        // looks like we take each of the query result row and map it to the Person
        // struct for each row. The query_map automatically takes each row obtained
        // as a result of the query and creates a iterator of the Ok(Person struct) 
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;



    for person in person_iter {
        // The unwrap will work on each of the Ok(person) and return the data inside
        // which is the Person struct
        // as noted in Rust help, unwrap function returns "T" type.
        println!("Found person {:?}", person.unwrap());
    }

    // if the above code does not return error return and empty Ok
    Ok(())
}
