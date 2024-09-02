#![allow(clippy::new_without_default)]

use std::time::Duration;

use crate::{
    api::send_danmu,
    ui::{
        footer::Footer, header::Header, helper::centered_rect, tabs::Tabs, AppState, InputMode,
        SliderBarState,
    },
    TuiState,
};
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, MouseEventKind};
use futures::StreamExt;
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{block::Title, Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
};
use tui_textarea::TextArea;

pub const MAX_INPUT_LENGTH: usize = 40;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub struct App {
    pub input_mode: InputMode,
    header: Header,
    footer: Footer,
    tabs: Tabs,
    pub textarea: TextArea<'static>,
    pub will_send_message: Vec<String>,
    err_text: Option<String>,
}

unsafe impl Send for App {}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        state: &mut TuiState,
    ) -> Result<()> {
        let mut interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND));
        let mut events = EventStream::new();

        if !self.will_send_message.is_empty() {
            self.send_danmu(state).await.unwrap_or_default();
        }

        tokio::select! {
            _ = interval.tick() => self.draw(terminal, state)?,
            Some(Ok(event)) = events.next() =>  self.handle_events(&event, state)?,
        }

        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>, state: &mut TuiState) -> Result<()> {
        terminal.draw(|f| f.render_stateful_widget(self, f.area(), state))?;

        Ok(())
    }

    pub async fn send_danmu(&mut self, state: &mut TuiState) -> Result<()> {
        self.err_text = None;
        let content = self.will_send_message.remove(0);
        let room_id = state.room_id;
        let cookie = state.cookie.clone();
        let will_send_message = content.clone();
        let result = tokio::spawn(async move {
            if let Some(cookie) = cookie {
                send_danmu(room_id, will_send_message.as_str(), cookie).await
            } else {
                Err("未登录".to_string())
            }
        })
        .await
        .unwrap();

        // if self.weared_v2.is_none() {
        //     let result = get_room_by_user(room_id).await;
        //     if result.is_ok() {
        //         self.weared_v2 = Some(result.unwrap().medal.curr_weared_v2);
        //     }
        // }

        if result.is_err() {
            self.err_text = Some(result.err().unwrap());
        } else {
            // let timestamp = Local::now().timestamp_millis();
            // let data = result.unwrap();
            // let mut user: User = User {
            //     uid: data.mode_info.user.uid,
            //     uname: data.mode_info.user.base.name,
            //     face: None,
            //     badge: None,
            //     identity: None,
            // };
            // if let Some(weared_v2) = &self.weared_v2 {
            //     user.badge = Some(Badge::new(
            //         weared_v2.name.clone(),
            //         weared_v2.level as u8,
            //         weared_v2.v2_medal_color_text.clone(),
            //         None,
            //         None,
            //         None,
            //     ));
            // }
            // let msg = DanmuMsg::new(user, content, timestamp, false, None);
            // state
            //     .messages
            //     .push((MsgType::Danmu, serde_json::to_string(&msg).unwrap()));
        }

        Ok(())
    }

    pub fn handle_events(&mut self, event: &Event, state: &mut TuiState) -> crate::app::Result<()> {
        match event {
            Event::Key(key) => match self.input_mode {
                InputMode::Normal if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Up if state.state == AppState::Running => self.previous_tab(),
                    KeyCode::Down if state.state == AppState::Running => self.next_tab(),
                    KeyCode::Char('w') if state.state == AppState::Running => self.scroll_up(),
                    KeyCode::Char('s') if state.state == AppState::Running => self.scroll_down(),
                    KeyCode::Char('q') if state.state == AppState::Running => state.quit(),
                    KeyCode::Char('q') if state.state == AppState::Quitting => state.quit(),
                    KeyCode::Char('y') if state.state == AppState::Quitting => {
                        state.state = AppState::Quit
                    }
                    KeyCode::Char('n') if state.state == AppState::Quitting => {
                        state.state = AppState::Running;
                    }
                    KeyCode::Char('t') if state.state == AppState::Running => {
                        self.toggle_slider_bar(state)
                    }
                    KeyCode::Enter
                        if state.state == AppState::Running && state.cookie.is_some() =>
                    {
                        self.input_mode = InputMode::Editing;
                        self.textarea
                            .set_style(Style::default().fg(Color::LightGreen));
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Up => {
                        self.previous_tab();
                    }
                    KeyCode::Down => {
                        self.next_tab();
                    }
                    KeyCode::Enter => {
                        if !self.textarea.lines()[0].is_empty() {
                            self.will_send_message
                                .push(self.textarea.lines()[0].to_string());
                            self.input_mode = InputMode::Normal;
                        }
                    }
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                    }
                    _ => {
                        if self.textarea.input(*key)
                            && self.textarea.lines()[0].len() > MAX_INPUT_LENGTH
                        {
                            self.textarea.delete_char();
                        }
                    }
                },
                _ => {}
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    self.scroll_down();
                }
                MouseEventKind::ScrollUp => {
                    self.scroll_up();
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    pub fn toggle_slider_bar(&mut self, state: &mut TuiState) {
        state.slider_bar_state = match state.slider_bar_state {
            SliderBarState::Normal => SliderBarState::Hiding,
            SliderBarState::Hiding => SliderBarState::Normal,
        };
    }

    pub fn scroll_up(&mut self) {
        if let Some(index) = self.tabs.state.selected() {
            let tab = &mut self.tabs.tabs[index];
            tab.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        if let Some(index) = self.tabs.state.selected() {
            let tab = &mut self.tabs.tabs[index];
            tab.scroll_down();
        }
    }

    pub fn next_tab(&mut self) {
        self.tabs.next_tab();
    }

    pub fn previous_tab(&mut self) {
        self.tabs.previous_tab();
    }
}

impl StatefulWidget for &mut App {
    type State = TuiState;

    fn render(self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.state == AppState::Quitting {
            self.render_quit_question(root, buf);
            return;
        }

        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);

        let horizontal: Layout = {
            if state.slider_bar_state == SliderBarState::Normal {
                Layout::horizontal([Constraint::Length(8), Constraint::Fill(1)])
            } else {
                Layout::horizontal([Constraint::Length(0), Constraint::Fill(1)])
            }
        };

        let [header_area, inner_area, footer_area] = vertical.areas(root);
        let [tabs_area, content_area] = horizontal.areas(inner_area);

        self.header.render(header_area, buf, state);

        if state.slider_bar_state == SliderBarState::Normal {
            self.render_tabs(tabs_area, buf);
        }

        self.render_selected_tab(content_area, buf, state);

        if self.input_mode == InputMode::Editing {
            self.render_input(footer_area, buf);
        } else {
            self.footer.render(footer_area, buf, state);
        }
    }
}

impl App {
    fn render_quit_question(&self, area: Rect, buf: &mut Buffer) {
        let area = centered_rect(60, 40, area);
        let block = Block::bordered()
            .title(" 提示 ")
            .title_alignment(Alignment::Center)
            .border_set(symbols::border::ROUNDED)
            .border_style(Style::default().fg(tailwind::ORANGE.c400))
            .padding(Padding::uniform(1));

        Paragraph::new(vec![
            Line::from("你确定要退出吗?".bold()).centered(),
            Line::raw(""),
            Line::raw(""),
            Line::from(vec![
                Span::from("确定 (按 Y/y)").bold().fg(tailwind::ORANGE.c300),
                Span::raw("    "),
                Span::from("取消 (按 N/n)").bold().fg(tailwind::RED.c300),
            ])
            .centered(),
        ])
        .block(block)
        .wrap(Wrap { trim: true })
        .render(area, buf);
    }

    fn render_tabs(&mut self, area: Rect, buf: &mut Buffer) {
        let highlight_style = Style::default()
            .bg(tailwind::YELLOW.c300)
            .fg(tailwind::BLACK)
            .bold();

        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1));

        let tabs: Vec<ListItem> = self
            .tabs
            .tabs
            .iter()
            .map(|tab| ListItem::from(tab.title()))
            .collect();

        let list = List::new(tabs)
            .block(block)
            .highlight_style(highlight_style)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::WhenSelected);

        StatefulWidget::render(list, area, buf, &mut self.tabs.state);
    }

    fn render_selected_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut TuiState) {
        let tab = if let Some(index) = self.tabs.state.selected() {
            &mut self.tabs.tabs[index]
        } else {
            self.tabs.state.select_first();
            &mut self.tabs.tabs[0]
        };

        tab.render(area, buf, state);
    }

    fn render_input(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea
            .set_placeholder_text("按 Enter 发送弹幕, Esc 取消输入");
        self.textarea
            .set_style(Style::default().fg(Color::LightCyan));
        let style = {
            if !self.textarea.lines()[0].is_empty() {
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::default())
                    .bold()
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            }
        };
        self.textarea.set_cursor_line_style(style);
        self.textarea.set_block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Color::LightGreen)
                .borders(Borders::ALL)
                .padding(Padding::left(1))
                .title({
                    if let Some(err_text) = &self.err_text {
                        Title::from(format!("错误: {} ", err_text).red())
                    } else {
                        Title::from(format!(
                            " {} / {} ",
                            self.textarea.lines()[0].len(),
                            MAX_INPUT_LENGTH
                        ))
                    }
                })
                .title_alignment(Alignment::Center),
        );

        self.textarea.render(area, buf);
    }
}
