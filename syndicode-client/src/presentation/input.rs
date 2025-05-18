mod exit;
mod list_detail;
mod main;
mod service_detail;

use super::{app::CurrentScreen, App};
use crate::domain::{auth::AuthenticationRepository, game::GameRepository};
use exit::handle_exit;
use list_detail::handle_list_detail;
use main::handle_main;
use ratatui::crossterm::event::Event;
use service_detail::handle_service_detail;

pub(super) async fn handle_crossterm_event<AUTH, GAME>(
    app: &mut App<'_, AUTH, GAME>,
    event: Event,
) -> anyhow::Result<()>
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    match app.current_screen {
        CurrentScreen::Main(_) => {
            handle_main(app, event).await;
        }
        CurrentScreen::ServiceDetail => {
            handle_service_detail(app, event).await?;
        }
        CurrentScreen::ListDetail => {
            handle_list_detail(app, event).await;
        }
        CurrentScreen::Exiting => {
            handle_exit(app, event);
        }
    };

    Ok(())
}
