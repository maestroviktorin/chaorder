use chaorder::{
    draw_new::Illustrator,
    illustration::ParseIllustration,
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, Terminal},
    TerminalOptions, Viewport,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 0, 10, 2)),
        },
    )?;
    terminal.clear()?;

    let illustration = ParseIllustration::from_str("1234567890\n0987654321");

    terminal.draw(|frame| {
        let area = frame.size();
        let ill = illustration.clone();

        frame.render_widget(Illustrator::new().illustration(ill).char_range(2).build(), area);
    })?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
