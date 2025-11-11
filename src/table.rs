use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Row, Table, TableState};
use crate::db_repo::{get_records, Records};

pub fn render_table(frame: &mut Frame, area: Rect, table_state: &mut TableState) {
    let header = Row::new(["Date", "Store", "Beer", "Allos","Comments"])
        .style(Style::new().bold())
        .bottom_margin(1);
    let records = get_records().unwrap_or_else(|_| Records::new(&vec!()));
    let rows = records.clone().records.iter().map(|r| Row::new(r.vec_of_fields())).collect::<Vec<Row>>();
    let cloned_records = records.clone();
    let footer = Row::new([
        format!("Total: {:.2}", cloned_records.all_total),
        format!("Store: {:.2}", cloned_records.store_total),
        format!("Beer: {:.2}", cloned_records.beer_total),
        format!("Allos: {:.2}", cloned_records.allos_total),
        "".to_string(),
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