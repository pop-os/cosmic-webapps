use crate::{
    common::{get_webapps, icon_cache_get, WebAppLauncher},
    gui::{Buttons, Message},
};

use cosmic::{
    iced::{Alignment, Length},
    iced_widget::Scrollable,
    style,
    widget::{self, text, Button, Column, Container, Row},
    Element,
};

#[derive(Debug, Clone)]
pub struct Home {
    pub edit_mode: bool,
    pub launcher: Option<WebAppLauncher>,
}

impl Home {
    pub fn new() -> Self {
        Home {
            edit_mode: false,
            launcher: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut app_list = Column::new();
        let webapps = get_webapps();

        for app in webapps.iter() {
            match app {
                Ok(data) => {
                    let num = Button::new(
                        Container::new(text(data.web_browser.name.clone()))
                            .center_x()
                            .center_y(),
                    )
                    .width(Length::FillPortion(1));

                    let app_name = Button::new(
                        Container::new(text(data.name.clone()))
                            .center_x()
                            .center_y(),
                    )
                    .width(Length::FillPortion(4))
                    .style(cosmic::theme::Button::Suggested);

                    let edit = widget::button(icon_cache_get("edit-symbolic", 16))
                        .on_press(Message::Clicked(Buttons::Edit(data.clone())))
                        .padding(8)
                        .style(style::Button::Icon);

                    let delete = widget::button(icon_cache_get("edit-delete-symbolic", 16))
                        .on_press(Message::Clicked(Buttons::Delete(data.clone())))
                        .padding(8)
                        .style(style::Button::Icon);

                    let mut row = Row::new().spacing(10).height(Length::Fixed(50.));
                    let mut row2 = Row::new().spacing(10).height(Length::Fixed(50.));

                    row = row.push(num);
                    row = row.push(app_name);

                    row2 = row2.push(edit);
                    row2 = row2.push(delete);
                    app_list = app_list.push(
                        Row::new()
                            .push(row)
                            .push(row2)
                            .width(Length::Fill)
                            .align_items(Alignment::Center)
                            .spacing(30),
                    );
                }
                Err(e) => tracing::error!("Error reading web app: {}", e),
            }
        }

        let mut installed = Column::new().spacing(20);

        if !webapps.is_empty() {
            installed = installed
                .push(text(format!("You have {} web apps installed:", webapps.len())).size(20));

            let scrollable_list = Scrollable::new(app_list).width(Length::Fill);

            installed = installed.push(scrollable_list);
        } else {
            installed = installed.push(
                text("You don't have any web app installed.\nPlease, press create button and create one.")
                    .size(20),
            );
        };

        Container::new(installed).padding(30).into()
    }
}
