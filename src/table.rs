use std::collections::HashSet;
use chrono::{NaiveDate};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Row, Table, TableState};
use crate::db_repo::{get_records, RecordsHolder};

pub fn render_table(frame: &mut Frame, area: Rect, table_state: &mut TableState) {
    let header = Row::new(["Date", "Store", "Beer", "Allos","Comments"])
        .style(Style::new().bold())
        .bottom_margin(1);
    let response = get_records().unwrap_or_else(|_| RecordsHolder::new(&vec!()));
    let rows = response.clone().records.iter().map(|r| Row::new(r.vec_of_fields())).collect::<Vec<Row>>();
    let dayz_total = response.records.iter().map(|r| r.date).collect::<HashSet<NaiveDate>>();
    let footer = Row::new([
        format!("Dayz : {}", dayz_total.len()),
        format!("Store: {:.2}", response.store_total),
        format!("Beer: {:.2}", response.beer_total),
        format!("Allos: {:.2}", response.allos_total),
        format!("Total: {:.2}", response.all_total),
    ]);


    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Fill(1),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(footer.italic())
        .column_spacing(1)
        .style(Color::Green)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::LightGreen)
        .cell_highlight_style(Style::new().reversed().light_yellow())
        .highlight_symbol("+++>   ");

    frame.render_stateful_widget(table, area, table_state);
}