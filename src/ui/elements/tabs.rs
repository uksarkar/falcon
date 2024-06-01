#[derive(Clone, Debug)]
pub struct TabNode {
    pub label: String,
    pub is_active: bool,
}

impl From<&str> for TabNode {
    fn from(value: &str) -> Self {
        TabNode {
            label: value.to_string(),
            is_active: false,
        }
    }
}

impl PartialEq<str> for TabNode {
    fn eq(&self, other: &str) -> bool {
        self.label == other
    }
}

#[derive(Default)]
pub struct Tabs {
    tabs: Vec<TabNode>,
    is_deactivated: bool,
}

impl Tabs {
    pub fn new(items: Vec<&str>, active_elm: &str) -> Self {
        Self {
            tabs: items
                .into_iter()
                .map(|elm| TabNode {
                    is_active: elm == active_elm,
                    label: elm.to_string(),
                })
                .collect(),
            is_deactivated: false,
        }
    }

    pub fn vec(&self) -> &Vec<TabNode> {
        &self.tabs
    }

    pub fn set_active(&mut self, active_elm: &str) {
        for node in &mut self.tabs {
            node.is_active = node.label == active_elm;
        }
    }

    pub fn is_active(&self) -> bool {
        !self.is_deactivated
    }

    pub fn deactivate(&mut self) {
        self.is_deactivated = true;
    }

    pub fn activate(&mut self) {
        self.is_deactivated = false;
    }

    pub fn toggle_activation(&mut self) {
        if self.is_deactivated {
            self.activate();
        } else {
            self.deactivate();
        }
    }

    pub fn get_active(&self) -> Option<&TabNode> {
        if self.is_deactivated {
            return None;
        }

        self.tabs.iter().find(|node| node.is_active)
    }
}

impl From<Vec<&str>> for Tabs {
    fn from(value: Vec<&str>) -> Self {
        Tabs {
            tabs: value.into_iter().map(|str| str.into()).collect(),
            is_deactivated: false,
        }
    }
}

#[macro_export]
macro_rules! create_tabs {
    ($items:expr, $msg:path, $minimize:expr, $label:expr) => {
        {
            use iced::{
                widget::{container, row, text, mouse_area, Row},
                Padding,
                Alignment
            };
            use crate::ui::app_theme::AppContainer;

            let mut items = $items.vec()
                .into_iter()
                .map(|node| {
                    if node.is_active {
                        return container(
                            mouse_area(
                                row![
                                    container("")
                                        .width(5)
                                        .height(5)
                                        .padding(Padding {
                                            bottom: 0.0,
                                            left: 5.0,
                                            right: 0.0,
                                            top: 0.0
                                        })
                                        .align_y(iced::alignment::Vertical::Center)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .center_x()
                                        .center_y()
                                        .style(AppContainer::SuccessIndicator),
                                    container(text(node.label.clone())).padding(Padding {
                                        top: 8.0,
                                        left: 5.0,
                                        right: 10.0,
                                        bottom: 8.0
                                    })
                                ]
                                .align_items(Alignment::Center),
                            ).interaction(iced::mouse::Interaction::Pointer)
                            .on_press($msg(node.clone())),
                        );
                    }

                    container(
                        mouse_area(text(node.label.clone())).interaction(iced::mouse::Interaction::Pointer).on_press($msg(node.clone())),
                    )
                    .padding(Padding::from([8, 10]))
                }).collect::<Vec<_>>();

            if let Some(message) = $minimize {
                items.push(container("").width(iced::Length::Fill));
                items.push(container(mouse_area(if let Some(txt) = $label {txt} else {container("-")}).on_press(message).interaction(iced::mouse::Interaction::Pointer)));
            }

            container(Row::from_vec(items.into_iter().map(|elm| elm.into()).collect()).width(iced::Length::Fill).align_items(Alignment::Center))
        }
    };
}
