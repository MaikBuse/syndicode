use super::{
    app::{App, CurrentScreen},
    widget::main_layout::MainLayoutWidget,
};
use crate::domain::{admin::AdminRepository, auth::AuthenticationRepository, game::GameRepository};
use ratatui::{
    layout::Margin,
    widgets::{StatefulWidget, Widget},
    Frame,
};

pub fn draw<'a, AUTH, ADMIN, GAME>(
    frame: &'a mut Frame<'_>,
    app: &'a mut App<'_, AUTH, ADMIN, GAME>,
) where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    let main_layout = MainLayoutWidget;
    let main_area = main_layout.render_and_get_areas(
        frame.area(),
        frame.buffer_mut(),
        app.maybe_username.clone(),
        app.is_stream_active,
    );

    app.service_list_widget.render(
        main_area.services.inner(Margin::new(1, 1)),
        frame.buffer_mut(),
        &mut app.service_list_state,
    );

    app.response_list_widget.render(
        main_area.responses,
        frame.buffer_mut(),
        &mut app.response_list_state,
        app.hide_game_tick_notification,
    );

    if let CurrentScreen::ServiceDetail = app.current_screen {
        if let Some(selected_service) = app.maybe_selected_service.as_mut() {
            app.service_detail_widget
                .render(frame.area(), frame.buffer_mut(), selected_service);
        }
    }

    if let CurrentScreen::ResponseDetail = app.current_screen {
        if let Some(response_detail_textarea) = app.maybe_response_detail_textarea.as_mut() {
            app.response_detail_widget.render(
                frame.area(),
                frame.buffer_mut(),
                response_detail_textarea,
            );
        }
    }

    if let CurrentScreen::Exiting = app.current_screen {
        app.exit_popup_widget
            .render(frame.area(), frame.buffer_mut());
    }
}
