mod db_repo;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    text::Line,
    widgets::{Block, Paragraph},
};
use ratatui::style::{Color, Style, Stylize};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Span;
use ratatui::widgets::{Row, Table, TableState};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();

        while self.running {
            terminal.draw(|frame| self.render(frame, &mut table_state))?;
            self.handle_crossterm_events(&mut table_state)?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame, table_state : &mut TableState) {

        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1)
            .split(frame.area());
        //let [top, main] = frame.area().layout(&layout);
        let title = Line::from_iter([
            Span::from("Table Widget").bold(),
            Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
        ]);
        frame.render_widget(title.centered(), layout[0]);


        self.render_table(frame,layout[1], table_state);
        // let title = Line::from("Ratatui Simple Template")
        //     .bold()
        //     .blue()
        //     .centered();
        // let text = "Hello, Ratatui!\n\n\
        //     Created using https://github.com/ratatui/templates\n\
        //     Press `Esc`, `Ctrl-C` or `q` to stop running.";
        // frame.render_widget(
        //     Paragraph::new(text)
        //         .block(Block::bordered().title(title))
        //         .centered(),
        //     frame.area(),
        // )
    }

    pub fn render_table(&mut self,frame: &mut Frame, area: Rect, table_state: &mut TableState) {
        let header = Row::new(["Ingredient", "Quantity", "Macros"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows = [
            Row::new(["Eggplant", "1 medium", "25 kcal, 6g carbs, 1g protein"]),
            Row::new(["Tomato", "2 large", "44 kcal, 10g carbs, 2g protein"]),
            Row::new(["Zucchini", "1 medium", "33 kcal, 7g carbs, 2g protein"]),
            Row::new(["Bell Pepper", "1 medium", "24 kcal, 6g carbs, 1g protein"]),
            Row::new(["Garlic", "2 cloves", "9 kcal, 2g carbs, 0.4g protein"]),
        ];
        let footer = Row::new([
            "Ratatouille Recipe",
            "",
            "135 kcal, 31g carbs, 6.4g protein",
        ]);
        let widths = [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(50),
        ];
        let table = Table::new(rows, widths)
            .header(header)
            .footer(footer.italic())
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(Style::new().reversed().yellow())
            .highlight_symbol("🍴 ");

        frame.render_stateful_widget(table, area, table_state);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self, table_state: &mut TableState) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key, table_state),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent, table_state: &mut TableState) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_ ,KeyCode::Char('j')) | (_,KeyCode::Down) => table_state.select_next(),
            (_ ,KeyCode::Char('k')) | (_ ,KeyCode::Up) => table_state.select_previous(),
            (_ ,KeyCode::Char('l')) | (_ ,KeyCode::Right) => table_state.select_next_column(),
            (_ ,KeyCode::Char('h')) | (_ ,KeyCode::Left) => table_state.select_previous_column(),
            (_ ,KeyCode::Char('g')) => table_state.select_first(),
            (_ ,KeyCode::Char('G')) => table_state.select_last(),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}


// fn main() {
//     println!("Hello, world!");
//     match get_records() {
//         Ok(_) => {}
//         Err(err) => {
//             println!("{}", err);
//         }
//     } ;
// }
