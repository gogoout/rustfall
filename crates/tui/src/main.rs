mod event;
mod render;
mod state;
mod tui;

fn main() -> anyhow::Result<()> {
    let mut tui = tui::Tui::try_new(false)?;
    tui.enter()?;
    tui.run()?;
    tui.exit()?;
    Ok(())
}
