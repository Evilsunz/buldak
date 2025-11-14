use chrono::NaiveDate;
use crate::db_repo::Record;

pub fn validate(input: &str, no_validation : bool ) -> String {
    if input.is_empty() ||
        no_validation  ||
        (input.contains("+") && input.find("+").unwrap() > 0) {
        //all ok
        return "".to_string()
    }
    if let Err(err) = input.parse::<f64>() {
        format!("{}", err)
    } else {
        "".to_string()
    }
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

pub fn into_record(store_price : &str,beer_price : &str,allos_price : &str,comments : &str,date : NaiveDate) -> Record {
    let mut store = if store_price.is_empty() {0.0} else { convert_to_f32(&store_price)};
    let mut beer = if beer_price.is_empty() {0.0} else { convert_to_f32(&beer_price)};
    let allos = if allos_price.is_empty() {0.0} else { convert_to_f32(&allos_price)};

    if beer < 0.0 {
        store += beer;
        beer = beer.abs();
    }
    store = format!("{:.2}", store).parse::<f32>().unwrap();
    Record {
        id: 0,
        store,
        beer,
        allos,
        comments: comments.to_string(),
        date: date,
    }
}