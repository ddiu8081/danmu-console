use chrono::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Padding, Paragraph},
};

use unicode_width::UnicodeWidthStr;

use crate::TuiState;

use super::{helper::render_basic_info, MsgType, UserActionMsg};

#[derive(Debug, Default)]
pub struct Footer;

impl StatefulWidget for &mut Footer {
    type State = TuiState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let enter = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::UserAction)
            .filter(|(_, msg)| {
                if let Ok(msg) = serde_json::from_str::<UserActionMsg>(msg) {
                    msg.action.as_str() == "enter"
                } else {
                    false
                }
            })
            .last();

        let enter_text = {
            if let Some(enter) = enter {
                let (_, msg) = enter;
                if let Ok(msg) = serde_json::from_str::<UserActionMsg>(msg) {
                    render_enter_text(msg)
                } else if state.cookie.is_some() {
                    Line::from(" 按 Enter 输入弹幕信息, Esc 取消输入 ")
                } else {
                    Line::from(" 登录后(Cookie)按 Enter 输入弹幕信息, Esc 取消输入 ")
                }
            } else if state.cookie.is_some() {
                Line::from(" 按 Enter 输入弹幕信息, Esc 取消输入 ")
            } else {
                Line::from(" 登录后(Cookie)按 Enter 输入弹幕信息, Esc 取消输入 ")
            }
        };

        let enter_text_width = enter_text.width() + 4;

        let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Min(60)])
            .flex(layout::Flex::SpaceBetween)
            .areas(area);

        let [left, right] = {
            if (enter_text_width as u16) < left.width {
                [left, right]
            } else {
                Layout::horizontal([Constraint::Length(0), Constraint::Fill(1)])
                    .flex(layout::Flex::SpaceBetween)
                    .areas(area)
            }
        };

        let watcher_text = format!("👀 {}", small_num_text(state.watchers));
        let attention_text = format!("🔥 {}", small_num_text(state.attention));
        let watcher_text_width = watcher_text.width() + 4;
        let attention_text_width = attention_text.width() + 4;

        let [info_area, watcher_area, attention_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(watcher_text_width as u16),
            Constraint::Length(attention_text_width as u16),
        ])
        .flex(layout::Flex::SpaceBetween)
        .areas(right);

        Paragraph::new(enter_text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1))
                    .title(if enter.is_some() && state.cookie.is_some() {
                        Title::from("按 Enter 输入弹幕信息, Esc 取消输入")
                    } else {
                        Title::from("提示")
                    })
                    .title_style(if enter.is_none() {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default()
                    })
                    .title_alignment(Alignment::Center),
            )
            .render(left, buf);

        if state.is_live {
            let text = format_duration(state.start_time);
            Paragraph::new(Line::from(vec![
                "🔴"
                    .to_string()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
                Span::from("  "),
                Span::from({
                    let live_text = format!("直播时长: {}", text);
                    if live_text.width() > info_area.width as usize {
                        text
                    } else {
                        live_text
                    }
                }),
            ]))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1))
                    .title(Title::from(format!(
                        " Start at {} ",
                        state.start_time.format("%H:%M")
                    )))
                    .title_style(Style::default().fg(Color::LightGreen))
                    .title_alignment(Alignment::Center),
            )
            .render(info_area, buf);
        } else {
            Paragraph::new(Line::from(vec![
                "⚫️"
                    .to_string()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
                Span::from("  "),
                Span::from("未开播").fg(Color::Red),
            ]))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .render(info_area, buf);
        }

        Paragraph::new(Line::from(watcher_text.fg(Color::LightGreen).bold()))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .render(watcher_area, buf);

        Paragraph::new(Line::from(attention_text.fg(Color::LightGreen).bold()))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .render(attention_area, buf);
    }
}

fn format_duration(start_time: NaiveDateTime) -> String {
    let now = Local::now().naive_local();
    let diff = now.signed_duration_since(start_time);
    let seconds = diff.num_seconds();

    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let secs = seconds - hours * 3600 - minutes * 60;

    let format_hours = {
        if hours > 0 {
            format!("{}", hours).to_string()
        } else {
            "".to_string()
        }
    };

    let format_minutes = {
        if hours > 0 {
            if minutes < 10 {
                format!("0{}", minutes).to_string()
            } else {
                format!("{}", minutes).to_string()
            }
        } else {
            format!("{}", minutes).to_string()
        }
    };

    let format_seconds = {
        if secs < 10 {
            format!("0{}", secs).to_string()
        } else {
            format!("{}", secs).to_string()
        }
    };

    if format_hours.is_empty() {
        format!("{}:{}", format_minutes, format_seconds).to_string()
    } else {
        format!("{}:{}:{}", format_hours, format_minutes, format_seconds).to_string()
    }
}

fn small_num_text(num: u32) -> String {
    if num > 10000 {
        format!("{:.1} 万", num as f32 / 10000.0).to_string()
    } else {
        format!("{}", num).to_string()
    }
}

fn render_enter_text(msg: UserActionMsg) -> Line<'static> {
    let mut spans = render_basic_info(None, msg.user, None);
    spans.push(Span::from("进入你的直播间"));

    Line::from(spans)
}
