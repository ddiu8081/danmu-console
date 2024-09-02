use std::fmt;

use chrono::{DateTime, Local, TimeZone, Utc};
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, ListState, Padding, Paragraph, Scrollbar, ScrollbarState},
};

use crate::TuiState;

use super::{
    helper::{
        render_danmu_message, render_gift_message, render_guard_buy_message,
        render_super_chat_message, render_user_action_message,
    },
    DanmuMsg, GiftMsg, GuardBuyMsg, MsgType, SliderBarState, SuperChatMsg, UserActionMsg,
};

#[derive(Clone)]
pub enum Tab {
    AllTab(usize, ScrollbarState),
    DanMuTab(usize, ScrollbarState),
    SCTab(usize, ScrollbarState),
    GiftTab(usize, ScrollbarState),
    CaptainTab(usize, ScrollbarState),
    EnterTab(usize, ScrollbarState),
}

impl Tab {
    pub fn title(&self) -> String {
        match self {
            Tab::AllTab(_, _) => "全部".to_string(),
            Tab::DanMuTab(_, _) => "弹幕".to_string(),
            Tab::SCTab(_, _) => " SC ".to_string(),
            Tab::GiftTab(_, _) => "礼物".to_string(),
            Tab::CaptainTab(_, _) => "上舰".to_string(),
            Tab::EnterTab(_, _) => "入场".to_string(),
        }
    }

    pub fn scroll_up(&mut self) {
        match self {
            Tab::AllTab(scroll, state) => {
                *scroll = scroll.saturating_sub(1);
                *state = state.position(*scroll);
            }
            Tab::DanMuTab(scroll, state) => {
                *scroll = scroll.saturating_sub(1);
                *state = state.position(*scroll);
            }
            Tab::SCTab(scroll, state) => {
                *scroll = scroll.saturating_sub(1);
                *state = state.position(*scroll);
            }
            Tab::GiftTab(scroll, state) => {
                *scroll = scroll.saturating_sub(1);
                *state = state.position(*scroll);
            }
            Tab::CaptainTab(scroll, state) => {
                *scroll = scroll.saturating_sub(1);
                *state = state.position(*scroll);
            }
            Tab::EnterTab(scroll, state) => {
                *scroll = scroll.saturating_sub(1);
                *state = state.position(*scroll);
            }
        };
    }

    pub fn scroll_down(&mut self) {
        match self {
            Tab::AllTab(scroll, state) => {
                *scroll = scroll.saturating_add(1);
                *state = state.position(*scroll);
            }
            Tab::DanMuTab(scroll, state) => {
                *scroll = scroll.saturating_add(1);
                *state = state.position(*scroll);
            }
            Tab::SCTab(scroll, state) => {
                *scroll = scroll.saturating_add(1);
                *state = state.position(*scroll);
            }
            Tab::GiftTab(scroll, state) => {
                *scroll = scroll.saturating_add(1);
                *state = state.position(*scroll);
            }
            Tab::CaptainTab(scroll, state) => {
                *scroll = scroll.saturating_add(1);
                *state = state.position(*scroll);
            }
            Tab::EnterTab(scroll, state) => {
                *scroll = scroll.saturating_add(1);
                *state = state.position(*scroll);
            }
        };
    }

    pub fn scroll(&self) -> usize {
        match self {
            Tab::AllTab(scroll, _) => *scroll,
            Tab::DanMuTab(scroll, _) => *scroll,
            Tab::SCTab(scroll, _) => *scroll,
            Tab::GiftTab(scroll, _) => *scroll,
            Tab::CaptainTab(scroll, _) => *scroll,
            Tab::EnterTab(scroll, _) => *scroll,
        }
    }

    pub fn set_state_content_length(&mut self, content_length: usize) {
        match self {
            Tab::AllTab(_, state) => *state = state.content_length(content_length),
            Tab::DanMuTab(_, state) => *state = state.content_length(content_length),
            Tab::SCTab(_, state) => *state = state.content_length(content_length),
            Tab::GiftTab(_, state) => *state = state.content_length(content_length),
            Tab::CaptainTab(_, state) => *state = state.content_length(content_length),
            Tab::EnterTab(_, state) => *state = state.content_length(content_length),
        };
    }

    pub fn set_state_viewport_content_length(&mut self, viewport_content_length: usize) {
        match self {
            Tab::AllTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::DanMuTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::SCTab(_, state) => *state = state.viewport_content_length(viewport_content_length),
            Tab::GiftTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::CaptainTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::EnterTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
        };
    }

    pub fn state(&mut self) -> &mut ScrollbarState {
        match self {
            Tab::AllTab(_, state) => state,
            Tab::DanMuTab(_, state) => state,
            Tab::SCTab(_, state) => state,
            Tab::GiftTab(_, state) => state,
            Tab::CaptainTab(_, state) => state,
            Tab::EnterTab(_, state) => state,
        }
    }
}

impl StatefulWidget for &mut Tab {
    type State = TuiState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        match self {
            Tab::AllTab(_, _) => self.render_all_tab(area, buf, state),
            Tab::DanMuTab(_, _) => self.render_danmu_tab(area, buf, state),
            Tab::SCTab(_, _) => self.render_sc_tab(area, buf, state),
            Tab::GiftTab(_, _) => self.render_gift_tab(area, buf, state),
            Tab::CaptainTab(_, _) => self.render_captain_tab(area, buf, state),
            Tab::EnterTab(_, _) => self.render_enter_tab(area, buf, state),
        }
    }
}

impl Tab {
    fn should_scroll_down(&mut self, content_length: usize, area: &Rect) -> bool {
        self.scroll() + area.height as usize - 2 < content_length
    }

    fn should_scroll_up(&mut self, area: &Rect, content_length: usize) -> bool {
        self.scroll() > 0 && area.height > content_length as u16
    }

    fn block(&self, state: &TuiState) -> Block {
        Block::bordered()
            .title({
                if state.slider_bar_state == SliderBarState::Hiding {
                    Title::from(format!(" {} ", self.title())).alignment(Alignment::Center)
                } else {
                    Title::from("")
                }
            })
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1))
    }

    fn setup_scrollbar(&mut self, len: usize, area: Rect) {
        self.set_state_content_length(len);
        while self.should_scroll_down(len, &area) {
            self.scroll_down();
        }
    }

    fn render_all_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t != MsgType::UserAction)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), true))
            .collect();

        self.setup_scrollbar(text.len(), area);

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll() as u16, 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_danmu_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::Danmu)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        self.setup_scrollbar(text.len(), area);

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll() as u16, 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_sc_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::SuperChat)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        self.setup_scrollbar(text.len(), area);

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll() as u16, 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_gift_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::Gift)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        self.setup_scrollbar(text.len(), area);

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll() as u16, 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_captain_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::GuardBuy)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        self.setup_scrollbar(text.len(), area);

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll() as u16, 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_enter_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::UserAction)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        self.setup_scrollbar(text.len(), area);

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll() as u16, 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }
}

impl Tab {
    fn render_msg(t: MsgType, b: String, render_type: bool) -> Line<'static> {
        match t {
            MsgType::Danmu => {
                if let Ok(msg) = serde_json::from_str::<DanmuMsg>(&b) {
                    let time = get_local_time_from_timestamp(msg.timestamp);

                    render_danmu_message(msg, time, render_type)
                } else {
                    Line::from(vec![Span::from("解析弹幕消息失败")])
                        .red()
                        .bold()
                }
            }
            MsgType::SuperChat => {
                if let Ok(msg) = serde_json::from_str::<SuperChatMsg>(&b) {
                    let time = get_local_time_from_timestamp(msg.timestamp);

                    render_super_chat_message(msg, time, render_type)
                } else {
                    Line::from(vec![Span::from("解析SC消息失败")]).red().bold()
                }
            }
            MsgType::Gift => {
                if let Ok(msg) = serde_json::from_str::<GiftMsg>(&b) {
                    let time = get_local_time_from_timestamp(msg.timestamp);

                    render_gift_message(msg, time, render_type)
                } else {
                    Line::from(vec![Span::from("解析礼物消息失败")])
                        .red()
                        .bold()
                }
            }
            MsgType::GuardBuy => {
                if let Ok(msg) = serde_json::from_str::<GuardBuyMsg>(&b) {
                    let time = get_local_time_from_timestamp(msg.timestamp);

                    render_guard_buy_message(msg, time, render_type)
                } else {
                    Line::from(vec![Span::from("解析上舰消息失败")])
                        .red()
                        .bold()
                }
            }
            MsgType::UserAction => {
                if let Ok(msg) = serde_json::from_str::<UserActionMsg>(&b) {
                    let time = get_local_time_from_timestamp(msg.timestamp);

                    render_user_action_message(msg, time, render_type)
                } else {
                    Line::from(vec![Span::from("解析用户操作消息失败")])
                        .red()
                        .bold()
                }
            }
        }
    }
}

pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub state: ListState,
}

impl Tabs {
    pub fn next_tab(&mut self) {
        let selected = self.state.selected().unwrap();
        if selected + 1 < self.tabs.len() {
            self.state.select(Some(selected + 1));
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn previous_tab(&mut self) {
        let selected = self.state.selected().unwrap();
        if selected > 0 {
            self.state.select(Some(selected - 1));
        } else {
            self.state.select(Some(self.tabs.len() - 1));
        }
    }
}

impl fmt::Debug for Tabs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tabs").finish()
    }
}

impl Default for Tabs {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            tabs: vec![
                Tab::AllTab(0, ScrollbarState::default()),
                Tab::DanMuTab(0, ScrollbarState::default()),
                Tab::SCTab(0, ScrollbarState::default()),
                Tab::GiftTab(0, ScrollbarState::default()),
                Tab::CaptainTab(0, ScrollbarState::default()),
                Tab::EnterTab(0, ScrollbarState::default()),
            ],
            state,
        }
    }
}

fn get_local_time_from_timestamp(timestamp: i64) -> DateTime<Local> {
    let time = DateTime::from_timestamp_millis(timestamp).unwrap_or(Utc::now());
    let time = time.naive_local();
    Local.from_utc_datetime(&time)
}
