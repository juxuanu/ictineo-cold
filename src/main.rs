mod nav_bar;
mod palette;

use crate::nav_bar::{nav_bar, NavEntry};
use crate::palette::{dark_palette, light_palette};
use iced::theme::Palette;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Fill, Padding, Theme};
use std::path::PathBuf;

fn theme(global_state: &GlobalState) -> Theme {
    Theme::custom(String::from("Adwaita"), {
        match global_state.current_theme {
            ThemeVariant::Dark => Palette {
                text: dark_palette::VIEW_FG,
                background: dark_palette::VIEW_BG,
                danger: dark_palette::DESTRUCTIVE,
                primary: dark_palette::ACCENT,
                success: dark_palette::SUCCESS,
            },
            ThemeVariant::Light => Palette {
                text: light_palette::VIEW_FG,
                background: light_palette::VIEW_BG,
                danger: light_palette::DESTRUCTIVE,
                primary: light_palette::ACCENT,
                success: light_palette::SUCCESS,
            },
        }
    })
}

fn main() -> iced::Result {
    iced::application("Ictineo (cold)", update, view)
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    ChangeNavEntry(NavEntry),
    NavigateBack,
    ChangeTheme(ThemeVariant),
}

#[derive(Default)]
pub(crate) struct GlobalState {
    current_nav_entry: NavEntry,
    current_path: PathBuf,
    current_theme: ThemeVariant,
}

#[derive(Default, Clone, Debug)]
enum ThemeVariant {
    #[default]
    Dark,
    Light,
}

fn update(global_state: &mut GlobalState, message: Message) {
    match message {
        Message::ChangeNavEntry(entry) => global_state.current_nav_entry = entry,
        Message::NavigateBack => {
            println!("navigated back")
        }
        Message::ChangeTheme(theme) => global_state.current_theme = theme,
    }
}

fn view(global_state: &GlobalState) -> Element<Message> {
    let top_bar = row![
        button(text("←").shaping(Shaping::Advanced)).on_press(Message::NavigateBack),
        button(text("→").shaping(Shaping::Advanced)),
        button(text("↑").shaping(Shaping::Advanced)),
        button(text("⟳").shaping(Shaping::Advanced)),
        container(
            text("Current path")
                .shaping(Shaping::Advanced)
                .align_y(Alignment::Center)
        )
        .width(Fill),
        button(text("🔍").shaping(Shaping::Advanced))
    ]
    .spacing(2)
    .padding(Padding::default().bottom(5));

    column![
        container(top_bar).center_x(Fill),
        container(row![
            nav_bar(global_state),
            container(text("Files view...")).width(Fill)
        ])
    ]
    .padding(3)
    .into()
}
