use crate::display::state::State;
use crate::display::tui::Tui;

mod display;
pub mod engine;

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::try_new()?;
    tui.enter()?;

    let (width, height) = tui.size()?;
    let mut state = State::new(width, height);

    while !state.should_quit {
        tui.draw(&mut state)?;
        state.update(tui.events.next()?);
    }

    tui.exit()?;
    Ok(())
}
