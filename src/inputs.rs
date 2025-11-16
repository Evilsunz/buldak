use crate::db_repo::{save_record};
use chrono::{NaiveDate, Utc};
use crossterm::event::{KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{Borders};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};
use tui_textarea::{TextArea};
use crate::input_validator::{into_record, validate};

/// App holds the state of the application
pub struct InputsState<'a> {
    /// Current input mode
    pub input_mode: InputMode,
    pub inputs: Vec<TextAreaHolder<'a>>,
    pub date_input: TextAreaHolder<'a>,
    pub selected_input_index: usize,
}

pub enum InputMode {
    Normal,
    Editing,
    DateEditing,
}

pub struct TextAreaHolder<'a> {
    pub text_area: TextArea<'a>,
    pub title: String,
    pub error_message: String,
    pub no_validation: bool
}

impl<'a> TextAreaHolder<'a> {
    pub fn new(title: &str) -> Self {
        TextAreaHolder {
            text_area: TextArea::default(),
            title: String::from(title),
            error_message: "".to_string(),
            no_validation: false
        }
    }

    pub fn new_validation_disabled(title: &str) -> Self {
        TextAreaHolder {
            text_area: TextArea::default(),
            title: String::from(title),
            error_message: "".to_string(),
            no_validation: true
        }
    }

    pub fn new_validation_disabled_with_value(title: &str, value : &str) -> Self {
        let holder = TextAreaHolder {
            text_area: TextArea::new(vec!(value.to_string())),
            title: String::from(title),
            error_message: "".to_string(),
            no_validation: true
        };
        holder
    }

    pub fn get_title(&self) -> String {
        if self.error_message.is_empty() {
            return self.title.clone();
        }
        self.error_message.clone()
    }

    pub fn get_block<'b>(&self) -> Block<'_> {
        if self.error_message.is_empty() {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green))
                .title(self.title.as_str()).clone()
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title_style(Color::Red)
                .title(self.title.as_str()).clone()
        }
    }

}

impl InputsState<'_> {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            inputs: vec![
                TextAreaHolder::new("Προϊόντα"),
                TextAreaHolder::new("Μπύρα"),
                TextAreaHolder::new("Αλλος"),
                TextAreaHolder::new_validation_disabled("Σχόλια")
            ],
            date_input: TextAreaHolder::new_validation_disabled_with_value("Ημερομηνία", Utc::now().date_naive().to_string().as_str()),
            selected_input_index: 0,
        }
    }

    pub fn inputs_to_default(&mut self){
        self.inputs = vec!(TextAreaHolder::new("Προϊόντα"),
                           TextAreaHolder::new("Μπύρα"),
                           TextAreaHolder::new("Αλλος"),
                           TextAreaHolder::new_validation_disabled("Σχόλια"));
    }

    pub fn move_cursor_to_next_input(&mut self) {
        let text_area = self.inputs.get_mut(self.selected_input_index).unwrap();
        text_area.error_message = validate(text_area.text_area.lines()[0].as_str(), text_area.no_validation);
        self.selected_input_index += 1;
        if self.selected_input_index >= self.inputs.len() {
            self.selected_input_index = 0;
        }
    }

    pub fn input(&mut self, key: KeyEvent) {
        let text_area = self.inputs.get_mut(self.selected_input_index).unwrap();
        text_area.text_area.input(key);
    }

    pub fn date_input(&mut self, key: KeyEvent) {
        self.date_input.text_area.input(key);
    }

    pub fn submit_message(&mut self) {
        let date = &self.date_input.text_area.lines()[0].clone();
        let store_price  = &self.inputs.get(0).unwrap().text_area.lines()[0].clone();
        let beer_price  = &self.inputs.get(1).unwrap().text_area.lines()[0].clone();
        let allos_price  = &self.inputs.get(2).unwrap().text_area.lines()[0].clone();
        let comments  = &self.inputs.get(3).unwrap().text_area.lines()[0].clone();

        let record = into_record(store_price, beer_price, allos_price , comments, date);

        let _ =save_record(&record);
        self.inputs_to_default()
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
        ]);
        let [help_area, input_area] = vertical.areas(area);
        let [date, left_input, center_input, right_input,comments_input] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .areas(input_area);
        self.render_help_area(frame, help_area);
        self.render_input_areas(frame, &[date, left_input, center_input, right_input, comments_input]);
        self.activate_input(frame, &[left_input, center_input, right_input, comments_input], date);
    }

    fn render_help_area(&self, frame: &mut Frame, area: Rect) {
        let (msg, style) = self.create_help_message();
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, area);
    }

    fn render_input_areas(&mut self, frame: &mut Frame, areas: &[Rect]) {
        self.create_date_input(frame, areas[0]);
        //frame.render_widget(&date, areas[0]);
        for (i, rect) in areas[1..].iter().enumerate() {
            let text_area_holder = &mut self.inputs.get_mut(i).unwrap();
            let block = text_area_holder.get_block();
            //TODO wtf clone ?????
            let text_area = &mut text_area_holder.text_area.clone();
            text_area.set_cursor_line_style(Style::default());
            if !text_area_holder.no_validation {
                text_area.set_placeholder_text("Εισαγάγετε ένα έγκυρο άθροισμα (π.χ. 1,56)");
            } else {
                text_area.set_placeholder_text("Προσθήκη σχολίων");
            }
            text_area.set_block(block);
            frame.render_widget(&*text_area, *rect);
        }
    }

    fn activate_input(&mut self, frame: &mut Frame, areas: &[Rect], date_area : Rect) {
        match self.input_mode {
            InputMode::Normal => {}
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => {
                let text_area_holder = &mut self.inputs.get_mut(self.selected_input_index).unwrap();
                let title = text_area_holder.get_title();
                let text_area = &mut text_area_holder.text_area;
                text_area
                    .set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
                text_area.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
                text_area.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Yellow))
                        .title(title),
                );
                frame.render_widget(&*text_area, areas[self.selected_input_index]);
            }
            InputMode::DateEditing => {
                let title = self.date_input.get_title();
                let text_area = &mut self.date_input.text_area;
                text_area
                    .set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
                text_area.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
                text_area.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Yellow))
                        .title(title),
                );
                frame.render_widget(&*text_area, date_area);
            }
        }
    }

    fn create_date_input(&mut self, frame: &mut Frame, area: Rect) {
        self.date_input.text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green))
            .title("Ημερομηνία").clone());
        self.date_input.text_area.set_style(Style::default());
        frame.render_widget(&self.date_input.text_area, area);
    }

    fn create_help_message(&self) -> (Vec<Span<'_>>, Style) {
        match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".green().into(),
                    "e".green().bold(),
                    " to start editing.".green(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".green().into(),
                    "Esc".green().bold(),
                    " to stop editing, ".green().into(),
                    "Enter".green().bold(),
                    " to record the expinses".green().into(),
                ],
                Style::default(),
            ),
            InputMode::DateEditing => (
                vec![
                    "Press ".green().into(),
                    "Esc".green().bold(),
                    " to stop editing, ".green().into(),
                    "Enter".green().bold(),
                    " to record the date".green().into(),
                ],
                Style::default(),
            ),
        }
    }

}
