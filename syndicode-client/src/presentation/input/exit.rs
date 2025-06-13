use crate::{
    domain::{
        admin::AdminRepository, auth::repository::AuthenticationRepository, game::GameRepository,
    },
    presentation::app::{App, CurrentScreen, CurrentScreenMain},
};
use crossterm::event::{Event, KeyCode};

pub(super) fn handle_exit<AUTH, ADMIN, GAME>(app: &mut App<'_, AUTH, ADMIN, GAME>, event: Event)
where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
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
