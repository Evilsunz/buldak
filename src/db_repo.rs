use chrono::{NaiveDate};
use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Record {
    pub id: i32,
    pub store: f32,
    pub beer: f32,
    pub allos: f32,
    pub comments: String,
    pub date: NaiveDate,
}

impl Record {
    pub fn vec_of_fields(&self) -> Vec<String> {
        vec!(
             self.date.format("%Y-%m-%d").to_string(),
             self.store.to_string(),
             self.beer.to_string(),
             self.allos.to_string(),
             self.comments.to_string(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct RecordsHolder {
    pub records: Vec<Record>,
    pub store_total : f32,
    pub beer_total : f32,
    pub allos_total : f32,
    pub all_total : f32,
}

impl RecordsHolder {

    pub fn new(recs: &Vec<Record>) -> RecordsHolder {
        let (store_total, beer_total, allos_total, all_total) = Self::calculate_totals(recs);
        RecordsHolder {
            records: recs.clone(),
            store_total,
            beer_total,
            allos_total,
            all_total,
        }
    }

    fn calculate_totals(recs: &[Record]) -> (f32, f32, f32, f32) {
        let (store_sum, beer_sum, allos_sum) = recs.iter().fold(
            (0.0, 0.0, 0.0),
            |(store, beer, allos), r| {
                (store + r.store, beer + r.beer, allos + r.allos)
            },
        );
        let total_sum = store_sum + beer_sum + allos_sum;
        (store_sum, beer_sum, allos_sum, total_sum)
    }

}

pub fn save_record(record: &Record) -> Result<usize> {
    let conn = get_connection();
    conn.execute(
        "INSERT INTO records (store,beer,allos,comment,date) VALUES (?1, ?2, ?3, ?4, ?5)",
        (record.store, record.beer, record.allos, &record.comments, &record.date ),
    )
}

pub fn delete_all() -> Result<usize> {
    let conn = get_connection();
    conn.execute(
        "delete from records",
        (),
    )
}

pub fn get_records() -> Result<RecordsHolder> {
    let conn = get_connection();

    let mut stmt = conn.prepare("SELECT id, store, beer, allos, comment, date FROM records")?;
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
    let records = person_iter.map(|r| r.unwrap()).collect::<Vec<Record>>();
    Ok(RecordsHolder::new(&records))
}

pub fn convert_to_f32(str : &str) -> f32{
    match str.find('+') {
        Some(_) => {
            let result = str.split("+").into_iter()
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<f32>().unwrap())
            .sum::<f32>();
            //TODO Oh my
            format!("{:.2}", result).parse::<f32>().unwrap()
        }
        None => {str.parse::<f32>().unwrap()}
    }
}

fn get_connection() -> Connection {
    let conn = Connection::open("./buldak.sqlite3").unwrap();
    let _ =conn.execute(
    "CREATE TABLE if not exists records (
                id    INTEGER PRIMARY KEY,
                store  FLOAT,
                beer  FLOAT,
                allos  FLOAT,
                comment  TEXT,
                date  TEXT
            )",
    (),
    );
    conn
}
