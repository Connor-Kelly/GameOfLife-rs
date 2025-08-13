use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, Paragraph},
};

pub fn render_help_popup(frame: &mut Frame) {
    let area = frame.area();
    let block = Block::bordered().title("Help (?)");
    let text = Paragraph::new(
        "
    ESC, Q, q - Exit
    SPACE     - Pause / Modify Mode
    ?         - Toggle Help
    ",
    )
    .block(block);
    let area = popup_area(area, 50, 30);
    frame.render_widget(Clear, area); //this clears out the background
    // frame.render_widget(block, area);
    frame.render_widget(text, area);
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
