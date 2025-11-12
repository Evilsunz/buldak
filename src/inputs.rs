use chrono::{NaiveDate, Utc};
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use crate::db_repo::Record;

/// App holds the state of the application
pub struct InputsState {
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
    pub inputs : Vec<Rect>,
    pub selected_input : Rect,
}

pub enum InputMode {
    Normal,
    Editing,
}

impl InputsState {
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
            inputs : vec!(),
            selected_input: Default::default(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_to_next_input(&mut self) {
        println!(" +++++++++++ Next")
        // let cursor_moved_right = self.character_index.saturating_add(1);
        // self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.store.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.store
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.store.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.store.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.store.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.store = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.store.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.messages.push(self.store.clone());
        self.store.clear();
        self.reset_cursor();
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
        self.render_cursor(frame);
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
        self.inputs.extend_from_slice(&areas[1..]);
        for &area in &areas[1..] {
            let input = self.create_input_paragraph();
            frame.render_widget(input, area);
        }
    }

    fn render_cursor(&self, frame: &mut Frame) {
        match self.input_mode {
            InputMode::Normal => {}
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => {
                if self.selected_input == Default::default() {
                    let input_area = self.inputs.get(0).unwrap();
                    frame.set_cursor_position(Position::new(
                        input_area.x + self.character_index as u16 + 1,
                        input_area.y + 1, ))
                }
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

    fn create_input_paragraph(&self) -> Paragraph {
        Paragraph::new(self.store.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default().green(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"))
    }

    fn create_date_paragraph(&self) -> Paragraph {
        Paragraph::new(self.date_now.to_string())
            .style(Style::default().green())
            .block(Block::bordered().title("Date"))
    }

}