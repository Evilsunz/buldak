use chrono::{NaiveDate};
use ratatui::prelude::{Stylize};
use ratatui::style::{Color};
use ratatui::widgets::{Block, Tabs};
use crate::App;
use crate::db_repo::{get_month_year_naive};


pub struct TabsState {
    pub months: Vec<NaiveDate>,
    pub index: usize,
}

impl TabsState {
    pub fn new(app: App) -> Self {
        let months = get_months();
        *app.current_month.lock().unwrap() = months[0].clone();
        Self { months, index: 0 }
    }

    pub fn select_next(&mut self, app: App) {
        self.index = (self.index + 1) % self.months.len();
        *app.current_month.lock().unwrap() = self.months[self.index].clone();
    }

    pub fn select_previous(&mut self, app: App) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.months.len() - 1;
        }
        *app.current_month.lock().unwrap() = self.months[self.index].clone();
    }
}

pub fn render_tabs(tabs_state: &mut TabsState) -> Tabs<'static> {
    let highlight_style = (Color::Black, Color::Yellow);
    tabs_state.months = get_months();

    let dates_str: Vec<String> = tabs_state.months.iter().map(|my|{
        my.format("%b-%Y").to_string()
    } ).collect();

    Tabs::new(dates_str)
        .green()
        .highlight_style(highlight_style)
        .select(tabs_state.index)
        .block(Block::bordered().border_style(Color::Green))
        .padding("++", "++")
        .divider(" ")
}

fn get_months() -> Vec<NaiveDate> {
    get_month_year_naive().unwrap_or_else(|_| panic!("Unable to retrieve dates from DB"))
}