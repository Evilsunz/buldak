use chrono::{NaiveDate, Utc};
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{Event, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use ratatui::layout::Rect;
use ratatui::widgets::{Borders, ListState};
use tui_textarea::{CursorMove, Input, TextArea};
use crate::db_repo::Record;

/// App holds the state of the application
pub struct InputsState<'a> {
    pub date_now : NaiveDate,
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
    pub inputs : Vec<TextAreaHolder<'a>>,
    pub selected_input_index : usize,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct TextAreaHolder<'a> {
    pub text_area: TextArea<'a>,
    pub title : String,
    pub error_message : String,
}

impl<'a> TextAreaHolder<'a> {
    pub fn new(title : &str) -> Self {
        TextAreaHolder{
            text_area :TextArea::default(),
            title : String::from(title),
            error_message : "".to_string(),
        }
    }
    pub fn get_title(&self) -> String{
        if self.error_message.is_empty() {
            return self.title.clone()
        }
        self.error_message.clone()
    }

}

impl InputsState<'_> {
    pub fn new() -> Self {
        Self {
            date_now : Utc::now().date_naive(),
            store: String::new(),
            beer: String::new(),
            allos: String::new(),
            comments: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            //inputs : vec!(TextArea::default(), TextArea::default(), TextArea::default()),
            inputs: vec!(TextAreaHolder::new("Προϊόντα"),TextAreaHolder::new("Beera"),TextAreaHolder::new("Allos")),
            selected_input_index: 0,
        }
    }

    pub fn move_cursor_to_next_input(&mut self) {
        let text_area = self.inputs.get_mut(self.selected_input_index).unwrap();
        Self::validate(text_area);
        self.selected_input_index += 1;
        if self.selected_input_index >= self.inputs.len() {
            self.selected_input_index = 0;
        }
    }

    pub fn enter_char(&mut self, key : KeyEvent) {
        let text_area = self.inputs.get_mut(self.selected_input_index).unwrap();
        let result =text_area.text_area.input(key);
    }

    pub fn submit_message(&mut self) {
        // self.messages.push(self.store.clone());
        // self.store.clear();
        // self.reset_cursor();
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(area);
        let [date, left_input, center_input, right_input] = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]).areas(input_area);
        self.render_help_area(frame, help_area);
        self.render_input_areas(frame, &[date ,left_input, center_input, right_input]);
        self.activate_input(frame, &[left_input, center_input, right_input]);
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
        // self.inputs.extend_from_slice();
        //todo separate logic by different fields

        for (i , rect) in areas[1..].iter().enumerate() {
            let text_area_holder = &mut self.inputs.get_mut(i).unwrap();
            let title = text_area_holder.get_title();
            let mut text_area = &mut text_area_holder.text_area;
            text_area.set_cursor_line_style(Style::default());
            text_area.set_placeholder_text("Enter a valid float (e.g. 1.56)");
            text_area.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Green))
                    .title(title)
            );
            frame.render_widget(&*text_area, *rect);
        }
    }

    fn activate_input(&mut self, frame: &mut Frame,areas: &[Rect]) {
        match self.input_mode {
            InputMode::Normal => {}
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => {
                let text_area_holder = &mut self.inputs.get_mut(self.selected_input_index).unwrap();
                let title = text_area_holder.get_title();
                let mut text_area = &mut text_area_holder.text_area;
                text_area.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
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

    fn validate(text_area: &mut TextAreaHolder) -> bool {
        let mut textarea = &mut text_area.text_area;
        if let Err(err) = textarea.lines()[0].parse::<f64>() {
            textarea.set_style(Style::default().fg(Color::LightRed));
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::LightRed),
            );
            text_area.error_message = format!("{}", err);
            false
        } else {
            textarea.set_style(Style::default().fg(Color::LightGreen));
            textarea.set_block(
                Block::default()
                    .border_style(Color::Green)
                    .borders(Borders::ALL)
                    .title("OK"),
            );
            text_area.error_message = "".to_string();
            true
        }
    }

}