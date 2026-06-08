use std::io::{self, Stdout};

use anyhow::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal;

/// Sets up a terminal for TUI (Text User Interface) rendering.
///
/// This function configures the terminal for interactive display by:
/// - Enabling raw mode (disables line buffering and echoing)
/// - Switching to the alternate screen buffer
/// - Enabling mouse capture for mouse events
///
/// # Returns
///
/// Returns a configured `Terminal` instance with a Crossterm backend on success.
///
/// # Errors
///
/// Returns an error if:
/// - Raw mode cannot be enabled
/// - The alternate screen or mouse capture fails to initialize
/// - Terminal creation fails
///
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

/// Restores the terminal to its original state after TUI usage.
///
/// This function cleans up terminal modifications by:
/// - Disabling raw mode
/// - Returning to the main screen buffer
/// - Disabling mouse capture
/// - Showing the cursor
///
/// # Arguments
///
/// * `terminal` - The terminal instance to restore (consumes ownership)
///
/// # Errors
///
/// Returns an error if any restoration step fails.
pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
