use chaorder::{
    draw_new::{Illustrator, IllustratorState},
    illustration::ParseIllustration,
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
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
    let mut state = IllustratorState::from((&terminal.get_frame().buffer_mut().to_owned(), &illustration));
    let illustrator = Illustrator::new().illustration(illustration.clone()).char_range(2).sleep_nanos(9000).build();

    // loop {
    //     terminal.draw(|frame| {
    //         frame.render_stateful_widget(&illustrator, frame.size(), &mut state);
    //     })?;
    // }

    terminal.draw(|frame| {
        frame.render_stateful_widget(&illustrator, frame.size(), &mut state);
    })?;

    // stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
