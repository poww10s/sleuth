use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Widget, Block, Borders},
    style::{Style, Color},
    Frame
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    // placeholder ui design 
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Min(0),
                Constraint::Percentage(40)
            ].as_ref()
        )
        .horizontal_margin(1)
        .split(f.size());
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(block, chunks[0]);
        let block = Block::default()
            .title("Controls")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(block, chunks[2]);
}

fn draw_bottom_text<B: Backend>(f: &mut Frame<B>, area: Rect) {
    
}

fn test_tables<B> (f: &mut Frame<B>, area: Rect) where B: Backend {

    

}