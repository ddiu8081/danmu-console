#[cfg(feature = "platform-napi")]
use std::{io::stdout, sync::Arc};

#[cfg(feature = "platform-napi")]
use crate::{
    api::get_room_info,
    app::App,
    ui::{AppState, MsgType},
};

use api::RoomInfo;
use chrono::NaiveDateTime;
#[cfg(feature = "platform-napi")]
use napi::bindgen_prelude::*;
#[cfg(feature = "platform-napi")]
use napi_derive::napi;

#[cfg(feature = "platform-napi")]
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

#[cfg(not(feature = "platform-napi"))]
use ratatui::crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
#[cfg(feature = "platform-napi")]
use tokio::sync::Mutex;
use ui::SliderBarState;

pub mod api;
pub mod app;
pub mod ui;

#[cfg(feature = "platform-napi")]
#[napi]
pub struct Tui {
    room_id: u32,
    app: Arc<Mutex<App>>,
    state: Arc<Mutex<TuiState>>,
}

#[cfg(feature = "platform-napi")]
#[napi]
impl Tui {
    #[napi(constructor)]
    pub fn new(room_id: u32, cookie: Option<String>) -> Result<Self> {
        Ok(Self {
            room_id,
            app: Arc::new(Mutex::new(App::default())),
            state: Arc::new(Mutex::new(TuiState::new(room_id, cookie))),
        })
    }

    #[napi(getter)]
    pub async fn state(&self) -> AppState {
        self.state.lock().await.state
    }

    /// # Safety
    /// This function is marked as unsafe because it requires exclusive access to the state.
    #[napi]
    pub async unsafe fn run(&mut self) -> Result<()> {
        let state = Arc::clone(&self.state);
        tokio::spawn(async move {
            let mut state = state.lock().await;
            let info = get_room_info(state.room_id).await.unwrap();
            state.update_info(info);
        })
        .await
        .unwrap_or_default();

        let state = Arc::clone(&self.state);
        let app = Arc::clone(&self.app);

        tokio::spawn(async move {
            let mut stdout = stdout();
            enable_raw_mode().unwrap();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
            let mut app_state = AppState::Running;
            while app_state != AppState::Quit {
                let mut state = state.lock().await;
                let mut app = app.lock().await;
                app.run(&mut terminal, &mut state).await.unwrap();
                app_state = state.state;
            }
        })
        .await
        .unwrap_or_default();

        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    /// # Safety
    /// This function is marked as unsafe because it requires exclusive access to the state.
    #[napi]
    pub async unsafe fn stop(&mut self) -> Result<()> {
        let mut state = self.state.lock().await;
        state.state = AppState::Quit;

        Ok(())
    }

    /// # Safety
    /// This function is marked as unsafe because it requires exclusive access to the state.
    #[napi]
    pub async unsafe fn send_attention_change(&mut self, attention: u32) {
        if attention == 1 {
            let info = get_room_info(self.room_id).await.unwrap();
            let mut state = self.state.lock().await;
            state.update_info(info);
        } else {
            let mut state = self.state.lock().await;
            state.update_attention(attention);
        }
    }

    /// # Safety
    /// This function is marked as unsafe because it requires exclusive access to the state.
    #[napi]
    pub async unsafe fn send_watcher_change(&mut self, watcher: u32) {
        let mut state = self.state.lock().await;
        state.update_watcher(watcher);
    }

    /// # Safety
    /// This function is marked as unsafe because it requires exclusive access to the state.
    #[napi]
    pub async unsafe fn send_live_change(&mut self, live: bool) {
        let mut state = self.state.lock().await;
        state.update_live(live);
    }

    /// # Safety
    /// This function is marked as unsafe because it requires exclusive access to the state.
    #[napi]
    pub async unsafe fn send_msg(&mut self, t: MsgType, msg: String) {
        let mut state = self.state.lock().await;
        state.messages.push((t, msg));
    }
}

#[cfg(feature = "platform-napi")]
#[napi]
pub fn restore_terminal() {
    let mut stdout = stdout();
    disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture).unwrap();
}

#[cfg(not(feature = "platform-napi"))]
pub fn init_panic_hook() {
    use std::panic::{set_hook, take_hook};

    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));
}

#[derive(Clone, Default, Debug)]
pub struct TuiState {
    pub cookie: Option<String>,
    pub slider_bar_state: SliderBarState,
    pub state: AppState,
    pub room_id: u32,
    pub attention: u32,
    pub watchers: u32,
    pub is_live: bool,
    pub start_time: NaiveDateTime,
    pub area_name: String,
    pub parent_area_name: String,
    pub title: String,
    pub messages: Vec<(MsgType, String)>,
}

impl TuiState {
    pub fn new(room_id: u32, cookie: Option<String>) -> Self {
        Self {
            room_id,
            cookie,
            slider_bar_state: SliderBarState::Hiding,
            ..Default::default()
        }
    }
}

impl TuiState {
    pub fn quit(&mut self) {
        if self.state == AppState::Quit {
            return;
        }

        if self.state == AppState::Quitting {
            self.state = AppState::Quit;
            return;
        }

        self.state = AppState::Quitting;
    }

    pub fn update_info(&mut self, info: RoomInfo) {
        self.area_name = info.area_name;
        self.parent_area_name = info.parent_area_name;
        self.title = info.title;
        self.attention = info.attention;
        self.is_live = info.live_status == 1;
        self.start_time =
            NaiveDateTime::parse_from_str(&info.live_time, "%Y-%m-%d %H:%M:%S").unwrap_or_default();
        self.watchers = info.online;
    }

    pub fn update_attention(&mut self, attention: u32) {
        self.attention = attention;
    }

    pub fn update_watcher(&mut self, watcher: u32) {
        self.watchers = watcher;
    }

    pub fn update_live(&mut self, live: bool) {
        self.is_live = live;
    }
}
