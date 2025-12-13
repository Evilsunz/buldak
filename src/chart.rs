use chrono::{Datelike, Duration, Month, NaiveDate, Utc};
use indexmap::IndexMap;
use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
};
use crate::db_repo::{get_records_holder, Record};

pub fn vertical_barchart(current_month : NaiveDate) -> BarChart<'static> {
    let records_holder = get_records_holder(current_month.clone()).unwrap();
    let bars: Vec<Bar> = create_time_serie(records_holder.records, current_month)
        .iter()
        .map(|(date, value)| vertical_bar(date, *value))
        .collect();
    let title = Line::from("Charts (Sums rounded)").style(Color::Green).centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(8)
}

fn vertical_bar(date: &str, expenses: f32) -> Bar<'static> {
    Bar::default()
        .value(expenses.round() as u64)
        .label(Line::from(format!("{}", date)).style(Color::Green))
        .text_value(format!("{:.0}",expenses))
        .style(temperature_style(expenses))
        .value_style(temperature_style(expenses).reversed())
}

fn temperature_style(_value: f32) -> Style {
    Style::new().fg(Color::Green)
}

fn create_time_serie(records : Vec<Record>, mut current_month: NaiveDate) -> IndexMap<String,f32>{
    let expenses = flatten_by_dates(&records);
    let mut serie :IndexMap<NaiveDate, f32> = IndexMap::new();
    for _i in 1..=32 {
        serie .insert(current_month, 0.0);
        current_month = current_month + Duration::days(1);
    }
    serie.extend(expenses);
    serie.sort_keys();
    let result :IndexMap<String,f32> =serie.into_iter().map(|(date, value)| {
        (date.format("%b-%d").to_string(), value)
    }).collect();
    result
}

fn flatten_by_dates(records: &[Record]) -> IndexMap<NaiveDate, f32> {
    let mut map = IndexMap::new();
    for record in records {
        *map.entry(record.date).or_default() += record.get_day_summary();
    }
    map
}