mod exit;
mod main;
mod response_detail;
mod service_detail;

use super::{app::CurrentScreen, App};
use crate::domain::{
    admin::AdminRepository, auth::repository::AuthenticationRepository, game::GameRepository,
};
use exit::handle_exit;
use main::handle_main;
use ratatui::crossterm::event::Event;
use response_detail::handle_response_detail;
use service_detail::handle_service_detail;

pub(super) async fn handle_crossterm_event<AUTH, ADMIN, GAME>(
    app: &mut App<'_, AUTH, ADMIN, GAME>,
    event: Event,
) -> anyhow::Result<()>
where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    match app.current_screen {
        CurrentScreen::Main(_) => {
            handle_main(app, event).await;
        }
        CurrentScreen::ServiceDetail => {
            handle_service_detail(app, event).await?;
        }
        CurrentScreen::ResponseDetail => {
            handle_response_detail(app, event).await;
        }
        CurrentScreen::Exiting => {
            handle_exit(app, event);
        }
    };

    Ok(())
}
