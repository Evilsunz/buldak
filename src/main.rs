mod db_repo;
mod table;
mod chart;
mod inputs;
mod input_validator;
mod tabs;

use std::sync::{Arc, Mutex};
use chrono::NaiveDate;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, text::Line};
use ratatui::style::{Stylize};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Span;
use ratatui::widgets::{TableState};
use crate::chart::vertical_barchart;
use crate::db_repo::{delete_all, get_records_holder, init_db};
use crate::inputs::{InputMode, InputsState};
use crate::table::render_table;
use crate::tabs::{render_tabs, TabsState};

fn main() -> color_eyre::Result<()> {
    init_db();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default, Clone)]
pub struct App {
    running: bool,
    current_month: Arc<Mutex<NaiveDate>>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        //TabsState
        let mut tabs_state = TabsState::new(self.clone());

        //Table
        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();

        //Inputs
        let mut inputs_state = InputsState::new();

        while self.running {
            terminal.draw(|frame| self.render(frame, &mut table_state, &mut inputs_state, &mut tabs_state))?;
            self.handle_crossterm_events(&mut table_state , &mut inputs_state , &mut tabs_state )?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, table_state : &mut TableState, inputs_state: &mut InputsState, tabs_state: &mut TabsState) {

        let main = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).split(frame.area());

        let inner = Layout::vertical([
            Constraint::Length(30),
            Constraint::Length(4),
            Constraint::Fill(1),
        ]).split(main[2]);

        let title = Line::from_iter([
            Span::from(format!("+++++ BULDAK expences +++++")).green().bold().underlined(),
        ]);
        frame.render_widget(title.centered(), main[0]);
        frame.render_widget(render_tabs(tabs_state), main[1]);
        //Table needs to maintain its own state (cursor movements so on)
        render_table(frame, inner[0], table_state, (self.current_month.lock().unwrap()).to_owned());
        inputs_state.render(frame, inner[1]);
        frame.render_widget(vertical_barchart(self.current_month.lock().unwrap().to_owned()), inner[2]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self, table_state: &mut TableState, inputs_state: &mut InputsState, tabs_state: &mut TabsState) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key, table_state, inputs_state, tabs_state),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent, table_state: &mut TableState, inputs_state: &mut InputsState, tabs_state: &mut TabsState) {
        match inputs_state.input_mode {
            InputMode::Normal => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Tab) => tabs_state.select_next(self.clone()),
                (_, KeyCode::Down) => table_state.select_next(),
                (_, KeyCode::Up) => table_state.select_previous(),
                (_, KeyCode::Right) => table_state.select_next_column(),
                (_, KeyCode::Left) => table_state.select_previous_column(),
                (_, KeyCode::Char('g')) => table_state.select_first(),
                (_, KeyCode::Char('G')) => table_state.select_last(),
                (_, KeyCode::Char('e')) => { inputs_state.input_mode = InputMode::Editing; },
                (_, KeyCode::Char('d')) => { inputs_state.input_mode = InputMode::DateEditing; },
                (_, KeyCode::Char('[')) => { let _ = delete_all(); },
                _ => {}
            }
            InputMode::Editing => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => { inputs_state.input_mode = InputMode::Normal;
                                       inputs_state.selected_input_index = 0 },
                (_, KeyCode::Enter) => { inputs_state.submit_message();
                                         inputs_state.inputs_to_default();
                                         inputs_state.input_mode = InputMode::Normal;
                                         inputs_state.selected_input_index = 0
                },
                (_, KeyCode::Tab) => { inputs_state.move_cursor_to_next_input(); },
                _ => { inputs_state.input(key); },
            }
            InputMode::DateEditing => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Enter) => { inputs_state.input_mode = InputMode::Normal },
                _ => { inputs_state.date_input(key); },
            }
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

}