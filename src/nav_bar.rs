use crate::{GlobalState, Message, ThemeVariant};
use iced::theme::palette::{Background, Pair};
use iced::widget::button::Style;
use iced::widget::{button, container, row, text, Column};
use iced::{Border, Color, Fill, Theme};

#[derive(Debug, Clone, Default)]
pub(crate) enum NavEntry {
    #[default]
    Home,
    Desktop,
    Documents,
    Downloads,
    Music,
    LocalDevices,
    Network,
}

impl NavEntry {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            NavEntry::Home => "Home",
            NavEntry::Desktop => "Desktop",
            NavEntry::Documents => "Documents",
            NavEntry::Downloads => "Downloads",
            NavEntry::Music => "Music",
            NavEntry::LocalDevices => "Devices",
            NavEntry::Network => "Network",
        }
    }
}

// todo: pass meaningful props for the component, instead of the whole state
pub(crate) fn nav_bar(state: &GlobalState) -> Column<Message> {
    iced::widget::column![
        container(
            button(NavEntry::Home.as_str())
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::Home))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::Home => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(
            button(NavEntry::Desktop.as_str())
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::Desktop))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::Desktop => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(
            button(NavEntry::Documents.as_str())
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::Documents))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::Documents => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(
            button(NavEntry::Downloads.as_str())
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::Downloads))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::Downloads => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(
            button(NavEntry::Music.as_str())
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::Music))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::Music => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(
            button(NavEntry::LocalDevices.as_str())
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::LocalDevices))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::LocalDevices => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(
            button("Network")
                .width(Fill)
                .on_press(Message::ChangeNavEntry(NavEntry::Network))
                .style(|theme: &Theme, status| {
                    match state.current_nav_entry {
                        NavEntry::Network => {
                            let mut active_style = button::primary(theme, status);
                            active_style.border = Border {
                                width: 1.0,
                                color: Color::WHITE,
                                radius: Default::default(),
                            };
                            active_style
                        }
                        _ => button::primary(theme, status),
                    }
                })
        )
        .width(Fill),
        container(row!(
            text("Theme"),
            button("Light").on_press(Message::ChangeTheme(ThemeVariant::Light)),
            button("Dark").on_press(Message::ChangeTheme(ThemeVariant::Dark))
        ))
        .width(Fill)
    ]
    .width(200)
    .spacing(10)
}
