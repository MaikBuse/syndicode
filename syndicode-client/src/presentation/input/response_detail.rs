use crate::{
    domain::{
        admin::AdminRepository, auth::repository::AuthenticationRepository, game::GameRepository,
    },
    presentation::{
        app::{App, CurrentScreen, CurrentScreenMain},
        widget::vim::{Transition, Vim},
    },
};
use crossterm::event::Event;

use super::utils::from_crossterm_into_ratatui;

pub(super) async fn handle_response_detail<AUTH, ADMIN, GAME>(
    app: &mut App<'_, AUTH, ADMIN, GAME>,
    event: Event,
) where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    let Some(response_detail_textarea) = app.maybe_response_detail_textarea.as_mut() else {
        tracing::error!("[Input] Failed to retrieve response list textarea");
        return;
    };

    let Ok(ratatui_event) = from_crossterm_into_ratatui(event) else {
        tracing::error!("Failed to parse crossterm event");
        return;
    };

    app.response_detail_vim = match app.response_detail_vim.transition(
        response_detail_textarea,
        ratatui_event.into(),
        &mut app.yank_buffer,
    ) {
        Transition::Mode(mode) if app.response_detail_vim.mode != mode => {
            response_detail_textarea.set_block(mode.block());
            response_detail_textarea.set_cursor_style(mode.cursor_style());
            Vim::new(mode)
        }
        Transition::Nop | Transition::Mode(_) => app.response_detail_vim.clone(),
        Transition::Pending(input) => app.response_detail_vim.clone().with_pending(input),
        Transition::Quit => {
            app.maybe_response_detail_textarea = None;
            app.current_screen = CurrentScreen::Main(CurrentScreenMain::Responses);
            return;
        }
    };
}
