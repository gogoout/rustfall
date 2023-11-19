use crate::display::tui::Tui;

mod display;
pub mod engine;

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::try_new(false)?;
    tui.enter()?;
    tui.run()?;
    tui.exit()?;
    Ok(())
}
