use crate::{
    domain::{auth::AuthenticationRepository, game::GameRepository},
    presentation::app::{App, CurrentScreen, CurrentScreenMain},
};
use ratatui::crossterm::event::{Event, KeyCode};

pub(super) fn handle_exit<AUTH, GAME>(app: &mut App<'_, AUTH, GAME>, event: Event)
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Char('y') => {
                app.should_exit = true;
            }
            KeyCode::Char('n') => {
                app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
            }
            _ => {}
        }
    }
}
