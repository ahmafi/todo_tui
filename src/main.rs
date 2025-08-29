use crate::apps::tui::TuiApp;

mod apps;
mod todo;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = TuiApp::new().run(terminal);
    ratatui::restore();
    result
}
