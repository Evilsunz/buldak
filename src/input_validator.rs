use crate::db_repo::Record;
use chrono::NaiveDate;

pub fn validate(input: &str, no_validation : bool ) -> String {
    if input.is_empty() ||
        no_validation  ||
        (input.contains("+") && input.find("+").unwrap() > 0) ||
        (input.starts_with("-") && input.len() > 1) {
        //all ok
        return String::new()
    }
    if let Err(err) = input.parse::<f64>() {
        format!("{}", err)
    } else {
        String::new()
    }
}

pub fn into_record(
    store_price: &str,
    beer_price: &str,
    allos_price: &str,
    comments: &str,
    date: &str,
) -> Record {
    let mut store = convert_to_f32(store_price);
    let mut beer = convert_to_f32(beer_price);
    let allos = convert_to_f32(allos_price);
    //Processing shortcut of -00.00 beer from store proce
    if beer < 0.0 {
        store += beer;
        beer = beer.abs();
    }
    store = format!("{:.2}", store).parse::<f32>().unwrap();

    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    Record {
        id: 0,
        store,
        beer,
        allos,
        comments: comments.to_string(),
        date: naive_date,
    }
}

fn convert_to_f32(str : &str) -> f32{
    if str.is_empty() {
        return 0.0
    }

    let (working_str, multiplier) = if str.starts_with('-') {
        (&str[1..], -1.0)
    } else {
        (str, 1.0)
    };

    let result = match working_str.find('+') {
        Some(_) => {
            let sanitized = working_str.replace(['(', ')'], "");

            let parse_term = |raw: &str| {
                raw.trim()
                    .parse::<f32>()
                    .expect("expected a numeric term in '+'-separated expression")
            };

            sanitized
                .split('+')
                .filter(|term| !term.trim().is_empty())
                .map(parse_term)
                .sum::<f32>()
        }
        None => {
            if working_str.is_empty() { 0.0 } else { working_str.parse::<f32>().unwrap() }
        }
    };

    let final_value = result * multiplier;
    format!("{:.2}", final_value).parse::<f32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_f32() {
        assert_eq!(convert_to_f32("10.5"), 10.5);
        assert_eq!(convert_to_f32("0"), 0.0);
        assert_eq!(convert_to_f32(""), 0.0);

        assert_eq!(convert_to_f32("10+5"), 15.0);
        assert_eq!(convert_to_f32("10.50+4.50"), 15.0);
        assert_eq!(convert_to_f32("1+1+1"), 3.0);

        assert_eq!(convert_to_f32("1.111+1.111"), 2.22);

        assert_eq!(convert_to_f32("10+"), 10.0);
        assert_eq!(convert_to_f32("+10"), 10.0);

        assert_eq!(convert_to_f32("-5+10"), -15.0);
        assert_eq!(convert_to_f32("-(5+10)"), -15.0);
        assert_eq!(convert_to_f32("-1.5"), -1.5);
        assert_eq!(convert_to_f32("-"), 0.0);
    }

    #[test]
    fn test_into_record_basic() {
        let record = into_record("10.50", "5.00", "1.00", "test comment", "2023-12-21");
        assert_eq!(record.store, 10.50);
        assert_eq!(record.beer, 5.00);
        assert_eq!(record.allos, 1.00);
        assert_eq!(record.comments, "test comment");
        assert_eq!(record.date.to_string(), "2023-12-21");
    }

    #[test]
    fn test_into_record_beer_shortcut() {
        let record = into_record("20.00", "-5.00", "0.0", "", "2023-12-21");
        assert_eq!(record.store, 15.00);
        assert_eq!(record.beer, 5.00);
    }

    #[test]
    fn test_into_record_with_summation() {
        let record = into_record("10+5.5", "2+2", "0", "calc", "2023-12-21");
        assert_eq!(record.store, 15.50);
        assert_eq!(record.beer, 4.00);
    }

    #[test]
    fn test_into_record_with_negative_summation() {
        let record = into_record("10+5", "-2+2+1+1", "0", "calc", "2023-12-21");
        assert_eq!(record.store, 9.00);
        assert_eq!(record.beer, 6.00);
    }

    #[test]
    #[should_panic]
    fn test_into_record_invalid_date() {
        into_record("10", "5", "0", "", "invalid-date");
    }
}
