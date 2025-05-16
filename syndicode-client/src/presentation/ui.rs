use super::{
    widget::{
        editing::EditingPopupWidget, exit::ExitPopupWidget, main_layout::MainLayoutWidget,
        response_list::ResponseListWidget, service_list::ServiceListWidget,
    },
    CurrentScreen, SelectedService,
};
use ratatui::widgets::ListState;
use ratatui::{
    layout::Margin,
    widgets::{ListState as RatatuiListState, StatefulWidget, Widget},
    Frame,
};

pub struct DrawStateData<'a> {
    pub maybe_username: Option<String>,
    pub service_list_widget: &'a ServiceListWidget,
    pub current_screen: CurrentScreen,
    pub editing_popup: &'a EditingPopupWidget,
    pub exit_popup: &'a ExitPopupWidget,
}

pub fn draw<'a>(
    frame: &'a mut Frame<'_>,
    draw_data: &'a DrawStateData<'a>,
    service_list_state: &'a mut ListState,
    maybe_selected_service: &'a mut Option<SelectedService<'_>>,
    response_list_widget: &'a ResponseListWidget,
) {
    let main_layout = MainLayoutWidget;
    let main_area = main_layout.render_and_get_areas(
        frame.area(),
        frame.buffer_mut(),
        draw_data.maybe_username.clone(),
    );

    draw_data.service_list_widget.render(
        main_area.services.inner(Margin::new(1, 1)),
        frame.buffer_mut(),
        service_list_state,
    );

    let mut ratatui_response_list_state = RatatuiListState::default();
    response_list_widget.render(
        main_area.responses,
        frame.buffer_mut(),
        &mut ratatui_response_list_state,
    );

    if let CurrentScreen::Editing = draw_data.current_screen {
        if let Some(selected_service_ref) = maybe_selected_service {
            draw_data
                .editing_popup
                .render(frame.area(), frame.buffer_mut(), selected_service_ref);
        }
    }

    if let CurrentScreen::Exiting = draw_data.current_screen {
        draw_data
            .exit_popup
            .render(frame.area(), frame.buffer_mut());
    }
}
