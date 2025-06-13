use crossterm::event::{Event, KeyCode};
use ratatui::crossterm::event as ratatui_event;

pub(super) fn from_crossterm_into_ratatui(event: Event) -> anyhow::Result<ratatui_event::Event> {
    let event: ratatui_event::Event = match event {
        Event::FocusGained => ratatui_event::Event::FocusGained,
        Event::FocusLost => ratatui_event::Event::FocusLost,
        Event::Key(key_event) => {
            let code = match key_event.code {
                KeyCode::Backspace => ratatui_event::KeyCode::Backspace,
                KeyCode::Enter => ratatui_event::KeyCode::Enter,
                KeyCode::Left => ratatui_event::KeyCode::Left,
                KeyCode::Right => ratatui_event::KeyCode::Right,
                KeyCode::Up => ratatui_event::KeyCode::Up,
                KeyCode::Down => ratatui_event::KeyCode::Down,
                KeyCode::Home => ratatui_event::KeyCode::Home,
                KeyCode::End => ratatui_event::KeyCode::End,
                KeyCode::PageUp => ratatui_event::KeyCode::PageUp,
                KeyCode::PageDown => ratatui_event::KeyCode::PageDown,
                KeyCode::Tab => ratatui_event::KeyCode::Tab,
                KeyCode::BackTab => ratatui_event::KeyCode::BackTab,
                KeyCode::Delete => ratatui_event::KeyCode::Delete,
                KeyCode::Insert => ratatui_event::KeyCode::Insert,
                KeyCode::F(f) => ratatui_event::KeyCode::F(f),
                KeyCode::Char(c) => ratatui_event::KeyCode::Char(c),
                KeyCode::Null => ratatui_event::KeyCode::Null,
                KeyCode::Esc => ratatui_event::KeyCode::Esc,
                KeyCode::CapsLock => ratatui_event::KeyCode::CapsLock,
                KeyCode::ScrollLock => ratatui_event::KeyCode::ScrollLock,
                KeyCode::NumLock => ratatui_event::KeyCode::NumLock,
                KeyCode::PrintScreen => ratatui_event::KeyCode::PrintScreen,
                KeyCode::Pause => ratatui_event::KeyCode::Pause,
                KeyCode::Menu => ratatui_event::KeyCode::Menu,
                KeyCode::KeypadBegin => ratatui_event::KeyCode::KeypadBegin,
                KeyCode::Media(media_key_code) => {
                    let media_key_code = match media_key_code {
                        crossterm::event::MediaKeyCode::Play => ratatui_event::MediaKeyCode::Play,
                        crossterm::event::MediaKeyCode::Pause => ratatui_event::MediaKeyCode::Pause,
                        crossterm::event::MediaKeyCode::PlayPause => {
                            ratatui_event::MediaKeyCode::PlayPause
                        }
                        crossterm::event::MediaKeyCode::Reverse => {
                            ratatui_event::MediaKeyCode::Reverse
                        }
                        crossterm::event::MediaKeyCode::Stop => ratatui_event::MediaKeyCode::Stop,
                        crossterm::event::MediaKeyCode::FastForward => {
                            ratatui_event::MediaKeyCode::FastForward
                        }
                        crossterm::event::MediaKeyCode::Rewind => {
                            ratatui_event::MediaKeyCode::Rewind
                        }
                        crossterm::event::MediaKeyCode::TrackNext => {
                            ratatui_event::MediaKeyCode::TrackNext
                        }
                        crossterm::event::MediaKeyCode::TrackPrevious => {
                            ratatui_event::MediaKeyCode::TrackPrevious
                        }
                        crossterm::event::MediaKeyCode::Record => {
                            ratatui_event::MediaKeyCode::Record
                        }
                        crossterm::event::MediaKeyCode::LowerVolume => {
                            ratatui_event::MediaKeyCode::LowerVolume
                        }
                        crossterm::event::MediaKeyCode::RaiseVolume => {
                            ratatui_event::MediaKeyCode::RaiseVolume
                        }
                        crossterm::event::MediaKeyCode::MuteVolume => {
                            ratatui_event::MediaKeyCode::MuteVolume
                        }
                    };

                    ratatui_event::KeyCode::Media(media_key_code)
                }
                KeyCode::Modifier(modifier_key_code) => {
                    let modifier_key_code = match modifier_key_code {
                        crossterm::event::ModifierKeyCode::LeftShift => {
                            ratatui_event::ModifierKeyCode::LeftShift
                        }
                        crossterm::event::ModifierKeyCode::LeftControl => {
                            ratatui_event::ModifierKeyCode::LeftControl
                        }
                        crossterm::event::ModifierKeyCode::LeftAlt => {
                            ratatui_event::ModifierKeyCode::LeftAlt
                        }
                        crossterm::event::ModifierKeyCode::LeftSuper => {
                            ratatui_event::ModifierKeyCode::LeftSuper
                        }
                        crossterm::event::ModifierKeyCode::LeftHyper => {
                            ratatui_event::ModifierKeyCode::LeftHyper
                        }
                        crossterm::event::ModifierKeyCode::LeftMeta => {
                            ratatui_event::ModifierKeyCode::LeftMeta
                        }
                        crossterm::event::ModifierKeyCode::RightShift => {
                            ratatui_event::ModifierKeyCode::RightShift
                        }
                        crossterm::event::ModifierKeyCode::RightControl => {
                            ratatui_event::ModifierKeyCode::RightControl
                        }
                        crossterm::event::ModifierKeyCode::RightAlt => {
                            ratatui_event::ModifierKeyCode::RightAlt
                        }
                        crossterm::event::ModifierKeyCode::RightSuper => {
                            ratatui_event::ModifierKeyCode::RightSuper
                        }
                        crossterm::event::ModifierKeyCode::RightHyper => {
                            ratatui_event::ModifierKeyCode::RightHyper
                        }
                        crossterm::event::ModifierKeyCode::RightMeta => {
                            ratatui_event::ModifierKeyCode::RightMeta
                        }
                        crossterm::event::ModifierKeyCode::IsoLevel3Shift => {
                            ratatui_event::ModifierKeyCode::IsoLevel3Shift
                        }
                        crossterm::event::ModifierKeyCode::IsoLevel5Shift => {
                            ratatui_event::ModifierKeyCode::IsoLevel5Shift
                        }
                    };

                    ratatui_event::KeyCode::Modifier(modifier_key_code)
                }
            };

            let modifiers = ratatui_event::KeyModifiers::from_bits(key_event.modifiers.bits())
                .ok_or(anyhow::anyhow!("Failed to parse modifier from bits"))?;

            let kind = match key_event.kind {
                crossterm::event::KeyEventKind::Press => ratatui_event::KeyEventKind::Press,
                crossterm::event::KeyEventKind::Repeat => ratatui_event::KeyEventKind::Repeat,
                crossterm::event::KeyEventKind::Release => ratatui_event::KeyEventKind::Release,
            };

            let state = ratatui_event::KeyEventState::from_bits(key_event.state.bits())
                .ok_or(anyhow::anyhow!("Failed to parse state from bits"))?;

            let ratatui_keyevent = ratatui_event::KeyEvent {
                code,
                modifiers,
                kind,
                state,
            };

            ratatui_event::Event::Key(ratatui_keyevent)
        }
        Event::Paste(paste_event) => ratatui_event::Event::Paste(paste_event),
        Event::Resize(x, y) => ratatui_event::Event::Resize(x, y),
        _ => return Err(anyhow::anyhow!("Event not supported")),
    };

    Ok(event)
}
