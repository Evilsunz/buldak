use chrono::{NaiveDate, Utc, NaiveDateTime, Duration};
use rand;
use rand::Rng;
use rusqlite::{Connection, Result, ToSql};

#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub store: f32,
    pub beer: f32,
    pub allos: f32,
    pub comments: String,
    pub date: NaiveDate,
}

pub fn get_records() -> Result<(Vec<Record>)> {
    let conn = Connection::open("./buldak.sqlite3")?;

    let mut stmt = conn.prepare("SELECT id, store, beer, allos, comment, date FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(0)?,
            store: row.get(1)?,
            beer: row.get(2)?,
            allos: row.get(3)?,
            comments: row.get(4)?,
            date: row.get(5)?,
        })
    })?;
    let result = person_iter.map(|r| r.unwrap()).collect::<Vec<Record>>();
    for person in result.iter().clone() {
        println!("Found record {:?}", person);
    }
    Ok((result))
}


// conn.execute(
// "CREATE TABLE person (
//             id    INTEGER PRIMARY KEY,
//             store  FLOAT,
//             beer  FLOAT,
//             allos  FLOAT,
//             comment  TEXT,
//             date  TEXT
//         )",
// (), // empty list of parameters.
// )?;
// let mut rng = rand::rng();
// for i in 1..20 {
// let me = Record {
// id: 0,
// store: f32::trunc(rng.random_range(0.0..100.0)  * 100.0) / 100.0,
// beer: f32::trunc(rng.random_range(0.0..100.0)  * 100.0) / 100.0,
// allos:f32::trunc(rng.random_range(0.0..100.0)  * 100.0) / 100.0,
// comments: String::new(),
// date: Utc::now().date_naive() + Duration::days(i),
// };
// conn.execute(
// "INSERT INTO person (store, beer, allos, comment, date) VALUES (?1, ?2, ?3, ?4, ?5)",
// (&me.store, &me.beer, &me.allos, &me.comments, &me.date),
// )?;
// }

