use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Stylize, Widget};
use ratatui::style::{Color, Style};
use ratatui::style::palette::tailwind;
use ratatui::symbols;
use ratatui::widgets::{Block, Padding, Tabs};

pub fn render_tabs() -> Tabs<'static> {
    // let titles = SelectedTab::iter().map(SelectedTab::title);
    let titles = vec!("Nov","Dec","Jan");
    //let highlight_style = (Color::default(), self.selected_tab.palette().c700);
    let highlight_style = (Color::Black, Color::Yellow);
    //let selected_tab_index = self.selected_tab as usize;
    Tabs::new(titles)
        .green()
        .highlight_style(highlight_style)
        .select(0)
        .block(Block::bordered()
            //.border_set(symbols::border::ROUNDED)
            //.padding(Padding::horizontal(1))
            .border_style(Color::Green))
        .padding("+", "+")
        .divider(" ")


    // Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
    //     .block(Block::bordered().title("Tabs"))
    //     .style(Style::default().green())
    //     .highlight_style(Style::default().yellow())
    //     .select(2)
    //     .divider(symbols::DOT)
    //     .padding("->", "<-")

}