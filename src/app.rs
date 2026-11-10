// SPDX-License-Identifier: MPL-2.0
use crate::config::Config;
use crate::fl;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::cosmic_theme::Spacing;
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{Alignment, Length, Limits, Subscription, window::Id};
use cosmic::prelude::*;
use cosmic::widget::{
    self, button, column, container, divider, grid, icon, row, space, text, toggler,
};
use jiff::{
    ToSpan, Zoned,
    civil::{Date, Weekday},
};
use tyme4rs::tyme::lunar::LunarDay;
use tyme4rs::tyme::solar::SolarDay;

#[derive(Default)]
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,
    config_helper: Option<cosmic_config::Config>,

    show_settings: bool,

    now: Zoned,
    date_today: Date,
    date_selected: Date,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    Tick,
    UpdateConfig(Config),
    ToggleShowCnCalendar(bool),
    ToggleShowTimeSecond(bool),
    ToggleShow24Hours(bool),
    ToggleShowHoliday(bool),
    ToggleSettings,
    PreviousMonth,
    NextMonth,
}
const LUNAR_DAY_CN: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];
pub fn format_lunar(lunar: &LunarDay) -> String {
    let lunar_month = lunar.get_month();
    let is_leap = lunar_month < 0;
    let m_abs = lunar_month.abs() as i8;

    // 优先节日
    if let Some(festival) = lunar.get_festival() {
        let full = festival.to_string();
        let name = full.split_whitespace().last().unwrap_or(&full);
        return name.to_string();
    }

    let month_str = match m_abs {
        1 => "正月",
        2 => "二月",
        3 => "三月",
        4 => "四月",
        5 => "五月",
        6 => "六月",
        7 => "七月",
        8 => "八月",
        9 => "九月",
        10 => "十月",
        11 => "冬月",
        12 => "腊月",
        _ => unreachable!("农历月份范围 1~12"),
    };

    let day = lunar.get_day();
    let res: String;
    if day == 1 {
        // 初一：显示月份（闰月带上闰）
        if is_leap {
            res = format!("闰{}", month_str);
        } else {
            res = month_str.to_string();
        }
    } else {
        // 其他日子只显示日
        if (1..=30).contains(&day) {
            res = LUNAR_DAY_CN[day - 1].to_string();
        } else {
            res = String::new();
        }
    }
    res
}
impl AppModel {
    fn save_config(&self) {
        if let Some(helper) = &self.config_helper {
            let _ = self.config.write_entry(helper);
        }
    }

    fn calendar_grid(&self) -> widget::Grid<'_, Message> {
        let mut calendar = grid().width(Length::Fill);
        let first_day_of_week = Weekday::Monday;

        let first_day = Date::new(self.date_selected.year(), self.date_selected.month(), 1)
            .expect("valid date");
        let num_days = first_day.weekday().since(first_day_of_week);
        let calendar_first = first_day.checked_sub(num_days.days()).expect("valid date");

        // 星期表头
        let weekdays = ["一", "二", "三", "四", "五", "六", "日"];
        for day_name in weekdays.iter() {
            calendar = calendar.push(
                text::caption(format!("周{}", day_name))
                    .apply(container)
                    .center_x(Length::Fixed(44.0)),
            );
        }
        calendar = calendar.insert_row();

        // 日历日期网格 (6行 x 7列 = 42天)
        for i in 0..42 {
            if i > 0 && i % 7 == 0 {
                calendar = calendar.insert_row();
            }

            let date = calendar_first
                .checked_add(i.days())
                .expect("valid date in calendar range");
            //let is_current_month = date.month() == self.date_selected.month();
            let is_today = date == self.date_today;
            let is_selected = date == self.date_selected;
            let label = if self.config.show_cn_calendar {
                let cn_calendar_day: LunarDay = SolarDay::from_ymd(
                    date.year() as isize,
                    date.month() as usize,
                    date.day() as usize,
                )
                .get_lunar_day();
                let lunar_str = if self.config.show_cn_calendar {
                    format_lunar(&cn_calendar_day)
                } else {
                    String::new()
                };
                format!("{}\n{}", date.day(), lunar_str)
            } else {
                date.day().to_string()
            };

            let style = if is_selected {
                cosmic::theme::Button::Suggested
            } else if is_today {
                cosmic::theme::Button::Standard
            } else {
                cosmic::theme::Button::Text
            };

            let btn = button::custom(text(label).apply(container).center(Length::Fill))
                .class(style)
                .height(Length::Fixed(44.0))
                .width(Length::Fixed(44.0));

            calendar = calendar.push(btn);
        }
        calendar
    }
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.alenx-zheng.cosmic-ext-applet-cn-calendar";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config_helper = cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok();
        let config = config_helper
            .as_ref()
            .and_then(|c| Config::get_entry(c).ok())
            .unwrap_or_default();

        let now = Zoned::now();
        let today = now.date();

        let app = AppModel {
            core,
            popup: None,
            config,
            config_helper,
            show_settings: false,
            now,
            date_today: today,
            date_selected: today,
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let period = if self.now.hour() < 12 {
            fl!("time-am")
        } else {
            fl!("time-pm")
        };

        let fmt = if self.config.show_time_second && self.config.show_24_hours {
            "%Y-%m-%d %H:%M:%S".to_string()
        } else if self.config.show_time_second && !self.config.show_24_hours {
            format!("%Y-%m-%d {} %I:%M:%S ", period)
        } else if !self.config.show_time_second && self.config.show_24_hours {
            "%Y-%m-%d %H:%M".to_string()
        } else {
            format!("%Y-%m-%d {} %I:%M ", period)
        };

        let time_str = self.now.strftime(&fmt).to_string();

        let content = widget::text(time_str)
            .width(Length::Shrink)
            .align_x(cosmic::iced::alignment::Horizontal::Center);

        self.core
            .applet
            .button_from_element(content, false)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let Spacing {
            space_xxs, space_s, ..
        } = cosmic::theme::active().cosmic().spacing;
        let date_str = self.date_selected.strftime("%Y年%m月").to_string();

        let month_controls = row![
            button::icon(icon::from_name("go-previous-symbolic"))
                .padding(8)
                .on_press(Message::PreviousMonth),
            button::icon(icon::from_name("go-next-symbolic"))
                .padding(8)
                .on_press(Message::NextMonth)
        ]
        .spacing(8);

        let settings_btn = button::icon(icon::from_name("emblem-system-symbolic"))
            .padding(8)
            .on_press(Message::ToggleSettings)
            .class(if self.show_settings {
                cosmic::theme::Button::Suggested
            } else {
                cosmic::theme::Button::Text
            });

        let settings_content = if self.show_settings {
            column![
                row![
                    text::body(fl!("show-cn-calendar")),
                    space::horizontal().width(Length::Fill),
                    toggler(self.config.show_cn_calendar).on_toggle(Message::ToggleShowCnCalendar)
                ]
                .align_y(Alignment::Center)
                .padding([8, 12]),
                row![
                    text::body(fl!("show-time-second")),
                    space::horizontal().width(Length::Fill),
                    toggler(self.config.show_time_second).on_toggle(Message::ToggleShowTimeSecond)
                ]
                .align_y(Alignment::Center)
                .padding([8, 12]),
                row![
                    text::body(fl!("show-cn-holiday")),
                    space::horizontal().width(Length::Fill),
                    toggler(self.config.show_cn_holiday).on_toggle(Message::ToggleShowHoliday)
                ]
                .align_y(Alignment::Center)
                .padding([8, 12]),
                row![
                    text::body(fl!("show-24-hours")),
                    space::horizontal().width(Length::Fill),
                    toggler(self.config.show_24_hours).on_toggle(Message::ToggleShow24Hours)
                ]
                .align_y(Alignment::Center)
                .padding([8, 12]),
                container(divider::horizontal::default()).padding([space_xxs, space_s]),
            ]
        } else {
            column![]
        };

        let content_list = column![
            row![
                settings_btn,
                space::horizontal().width(Length::Fill),
                month_controls
            ]
            .align_y(Alignment::Center)
            .padding([12, 20]),
            settings_content,
            column![
                text(date_str).size(18),
                self.calendar_grid().padding([0, 12].into())
            ]
            .spacing(8),
        ]
        .padding([8, 0]);

        self.core
            .applet
            .popup_container(container(content_list))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let period = if self.config.show_time_second { 1 } else { 60 };

        Subscription::batch(vec![
            cosmic::iced::time::every(std::time::Duration::from_secs(period))
                .map(|_| Message::Tick),
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
        ])
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::Tick => {
                self.now = Zoned::now();
                Task::none()
            }
            Message::UpdateConfig(config) => {
                self.config = config;
                Task::none()
            }
            Message::ToggleShowCnCalendar(val) => {
                self.config.show_cn_calendar = val;
                self.save_config();
                Task::none()
            }
            Message::ToggleShowHoliday(val) => {
                self.config.show_cn_holiday = val;
                self.save_config();
                Task::none()
            }
            Message::ToggleShow24Hours(val) => {
                self.config.show_24_hours = val;
                self.save_config();
                Task::none()
            }
            Message::ToggleShowTimeSecond(val) => {
                self.config.show_time_second = val;
                self.save_config();
                Task::none()
            }
            Message::ToggleSettings => {
                self.show_settings = !self.show_settings;
                Task::none()
            }
            Message::PreviousMonth => {
                if let Ok(date) = self.date_selected.checked_sub(1.month()) {
                    self.date_selected = date;
                }
                Task::none()
            }
            Message::NextMonth => {
                if let Ok(date) = self.date_selected.checked_add(1.month()) {
                    self.date_selected = date;
                }
                Task::none()
            }
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    self.date_today = self.now.date();
                    self.date_selected = self.date_today;
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    return get_popup(popup_settings);
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
                Task::none()
            }
        }
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
