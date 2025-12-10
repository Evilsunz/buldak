use chrono::{Datelike, Utc};
use ratatui::prelude::{Stylize, Widget};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Padding, Tabs};
use crate::App;
use crate::db_repo::get_month_year;


pub struct TabsState {
    pub months: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(mut app: App) -> Self {
        let mut months = get_month_year().unwrap_or_else(|_| vec!["Unable to retrieve dates from DB".to_string()]);
        let current_month = Utc::now().format("%m-%Y").to_string();

        if !months.contains(&current_month) {
            months.insert(0, current_month);
        }
        app.current_month = months[0].clone();
        Self { months, index: 0 }
    }

    pub fn select_next(&mut self, mut app: App) {
        self.index = (self.index + 1) % self.months.len();
        app.current_month = self.months[self.index].clone();
    }

    pub fn select_previous(&mut self, mut app: App) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.months.len() - 1;
        }
        app.current_month = self.months[self.index].clone();
    }
}

pub fn render_tabs(tabs_state: &mut TabsState) -> Tabs<'static> {
    let highlight_style = (Color::Black, Color::Yellow);
    Tabs::new(tabs_state.months.clone())
        .green()
        .highlight_style(highlight_style)
        .select(tabs_state.index)
        .block(Block::bordered().border_style(Color::Green))
        .padding("++", "++")
        .divider(" ")
}