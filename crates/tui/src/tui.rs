use std::time::{Duration, Instant};
use std::{io, panic};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::event::EventHandler;
use crate::render::Renderer;
use crate::state::State;

pub type CrosstermTerminal = Terminal<CrosstermBackend<io::Stderr>>;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
    renderer: Renderer,
    state: State,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn try_new(no_braille: bool) -> anyhow::Result<Self> {
        let backend = CrosstermBackend::new(io::stderr());

        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new();
        let renderer = Renderer::new(no_braille);

        let rect = terminal.size()?;
        let state = State::new(rect.width as usize, rect.height as usize, no_braille);

        Ok(Self {
            terminal,
            events,
            renderer,
            state,
        })
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn enter(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        const TICK_RATE: Duration = Duration::from_micros(16667);
        // Why do I need to have this offset here??
        // Remove this cause the fps to stay consistent at 50 fps
        const OFFSET: Duration = Duration::from_millis(5);

        let mut last_tick = Instant::now();
        while !self.state.should_quit {
            if let Some(e) = TICK_RATE
                .checked_sub(last_tick.elapsed() + OFFSET)
                .and_then(|timeout| self.events.next(timeout).transpose())
                .transpose()?
            {
                self.state.update(e);
            }

            if last_tick.elapsed() >= TICK_RATE {
                last_tick = Instant::now();
                self.draw()?;
                self.state.tick();
            }
        }

        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.terminal
            .draw(|frame| self.renderer.render(&self.state, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> anyhow::Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> anyhow::Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
