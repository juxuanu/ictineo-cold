use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Fill};

fn main() -> iced::Result {
    iced::application("A cool counter", update, view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    let tabs = row![button(text("Tab 1"))
        .style(|_, _| button::Style::default().with_background(Color::TRANSPARENT))];
    let top_bar = row![
        button("<-"),
        button("->"),
        button("^"),
        button("R"),
        container(text("Current path")).width(Fill),
        button("S")
    ];
    let nav = column![
        container(button("Home")).width(Fill),
        container(button("Desktop")).width(Fill),
        container(button("Documents")).width(Fill),
        container(button("Downloads")).width(Fill),
        container(button("Music")).width(Fill),
        container(button("Root")).width(Fill),
        container(button("External SSD")).width(Fill),
        container(button("Google Drive")).width(Fill),
        container(button("Network")).width(Fill),
    ]
    .width(300)
    .spacing(10);

    column![
        container(tabs),
        container(top_bar).center_x(Fill),
        container(row![nav, container(text("Files view...")).width(Fill)]).center_x(Fill)
    ]
    .into()
}
