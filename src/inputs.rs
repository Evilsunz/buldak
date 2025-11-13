use crate::db_repo::{convert_to_f32, save_record, Record};
use chrono::{NaiveDate, Utc};
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{Event, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{Borders, ListState};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
};
use tui_textarea::{CursorMove, Input, TextArea};

/// App holds the state of the application
pub struct InputsState<'a> {
    pub date_now: NaiveDate,
    pub store: String,
    pub beer: String,
    pub allos: String,
    pub comments: String,
    /// Position of cursor in the editor area.
    pub character_index: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
    pub inputs: Vec<TextAreaHolder<'a>>,
    pub selected_input_index: usize,
}

pub enum InputMode {
    Normal,
    Editing,
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

    pub fn get_title(&self) -> String {
        if self.error_message.is_empty() {
            return self.title.clone();
        }
        self.error_message.clone()
    }

    pub fn get_block<'b>(&self) -> Block {
        if self.error_message.is_empty() {
            return Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green))
                .title(self.title.as_str()).clone()
        } else {
            return Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightRed)
                .title_style(Color::Red)
                .title(self.title.as_str()).clone()
        }
    }

    fn validate(&mut self) -> bool {
        if self.text_area.lines()[0].is_empty() || self.no_validation {
            //all ok
            self.error_message = "".to_string();
            return true
        }
        if let Err(err) = self.text_area.lines()[0].parse::<f64>() {
            self.error_message = format!("{}", err);
            false
        } else {
            self.error_message = "".to_string();
            true
        }
    }
}

impl InputsState<'_> {
    pub fn new() -> Self {
        Self {
            date_now: Utc::now().date_naive(),
            store: String::new(),
            beer: String::new(),
            allos: String::new(),
            comments: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            inputs: vec![
                TextAreaHolder::new("Προϊόντα"),
                TextAreaHolder::new("Μπύρα"),
                TextAreaHolder::new("Αλλος"),
                TextAreaHolder::new_validation_disabled("Σχόλια")
            ],
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
        text_area.validate();
        self.selected_input_index += 1;
        if self.selected_input_index >= self.inputs.len() {
            self.selected_input_index = 0;
        }
    }

    pub fn enter_char(&mut self, key: KeyEvent) {
        let text_area = self.inputs.get_mut(self.selected_input_index).unwrap();
        let result = text_area.text_area.input(key);
    }

    pub fn submit_message(&mut self) {
        let store_price  = self.inputs.get(0).unwrap().text_area.lines()[0].clone();
        let beer_price  = self.inputs.get(1).unwrap().text_area.lines()[0].clone();
        let allos_price  = self.inputs.get(2).unwrap().text_area.lines()[0].clone();
        let comments  = self.inputs.get(3).unwrap().text_area.lines()[0].clone();
        let mut store = if store_price.is_empty() {0.0} else { convert_to_f32(&store_price)};
        let mut beer = if beer_price.is_empty() {0.0} else { convert_to_f32(&beer_price)};;
        let allos = if allos_price.is_empty() {0.0} else { convert_to_f32(&allos_price)};;

        if beer < 0.0 {
            store += beer;
            beer = beer.abs();
        }

        let record = Record {
            id: 0,
            store,
            beer,
            allos,
            comments,
            date: self.date_now,
        };
        save_record(&record);
        self.inputs_to_default()
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(area);
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
        self.activate_input(frame, &[left_input, center_input, right_input, comments_input]);
        self.render_messages_area(frame, messages_area);
    }

    fn render_help_area(&self, frame: &mut Frame, area: Rect) {
        let (msg, style) = self.create_help_message();
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, area);
    }

    fn render_input_areas(&mut self, frame: &mut Frame, areas: &[Rect]) {
        let date = self.create_date_paragraph();
        frame.render_widget(date, areas[0]);
        for (i, rect) in areas[1..].iter().enumerate() {
            let mut text_area_holder = &mut self.inputs.get_mut(i).unwrap();
            let block = text_area_holder.get_block();
            //TODO wtf clone ?????
            let mut text_area = &mut text_area_holder.text_area.clone();
            text_area.set_cursor_line_style(Style::default());
            if !text_area_holder.no_validation {
                text_area.set_placeholder_text("Enter a valid summ (e.g. 1.56)");
            }
            text_area.set_block(block);
            frame.render_widget(&*text_area, *rect);
        }
    }

    fn activate_input(&mut self, frame: &mut Frame, areas: &[Rect]) {
        match self.input_mode {
            InputMode::Normal => {}
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => {
                let text_area_holder = &mut self.inputs.get_mut(self.selected_input_index).unwrap();
                let title = text_area_holder.get_title();
                let mut text_area = &mut text_area_holder.text_area;
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
        }
    }

    fn render_messages_area(&self, frame: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, area);
    }

    fn create_help_message(&self) -> (Vec<Span>, Style) {
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
                    " to record the message".green().into(),
                ],
                Style::default(),
            ),
        }
    }

    fn create_date_paragraph(&self) -> Paragraph {
        Paragraph::new(self.date_now.to_string())
            .style(Style::default().green())
            .block(Block::bordered().title("Date"))
    }

}
