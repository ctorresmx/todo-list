use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, List, ListDirection},
    DefaultTerminal, Frame,
};

use crate::persistance;

pub fn tui() {
    color_eyre::install().unwrap();
    let terminal = ratatui::init();
    let _result = run(terminal);
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let todos = persistance::read();
    let list = List::new(todos.into_iter().map(|todo| todo.content))
        .block(Block::bordered().title("Todos"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    frame.render_widget(list, frame.area());
}
