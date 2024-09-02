use std::str::FromStr;

use chrono::prelude::*;
use ratatui::{prelude::*, style::palette::tailwind};

use super::{
    colors::{GRAY_COLOR, USER_COLORS},
    DanmuMsg, GiftMsg, GuardBuyMsg, MsgType, SuperChatMsg, User, UserActionMsg,
};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

/// replace emoji to unicode
pub fn replace_emoji_to_unicode(emoji_str: &str) -> String {
    emoji_str
        .replace("[dog]", "🐶")
        .replace("[手机]", "📱")
        .replace("[花]", "🌹")
        .replace("[吃瓜]", "🍉")
        .replace("[比心]", "❤️")
}

pub fn render_basic_info(
    t: Option<MsgType>,
    user: User,
    time: Option<DateTime<Local>>,
) -> Vec<Span<'static>> {
    let mut spans = vec![];

    let color = *GRAY_COLOR.get_or_init(|| Color::from_hsl(0.0, 0.0, 40.0));

    if let Some(time) = time {
        spans.push(Span::from(format!("{}", time.format("%H:%M:%S"))).fg(color));
    }

    if let Some(t) = t {
        spans.push(Span::from(format!(" [{}] ", t)).fg(Color::LightYellow));
    } else {
        spans.push(Span::raw(" "));
    }

    if let Some(ref badge) = user.badge {
        let mut color = *GRAY_COLOR.get().unwrap();
        if let Some(ref anchor) = badge.anchor {
            if let Some(is_same_room) = anchor.is_same_room {
                if is_same_room {
                    color = Color::from_str(&badge.color).unwrap_or(color)
                }
            }
        }

        spans.push(Span::from(format!(" {} ", badge.name)).bg(color));
        spans.push(
            Span::from(format!(" {} ", badge.level))
                .fg(color)
                .bg(Color::White),
        );

        spans.push(Span::raw(" "));
    }

    if let Some(identity) = user.identity {
        let index = identity.guard_level as usize % USER_COLORS.len();
        let color =
            Color::from_str(USER_COLORS[index]).unwrap_or(Color::from_str(USER_COLORS[0]).unwrap());

        spans.push(Span::from(user.uname).bold().fg(color));
    } else {
        spans.push(
            Span::from(user.uname)
                .bold()
                .fg(Color::from_str(USER_COLORS[0]).unwrap()),
        );
    }

    spans.push(Span::raw(": "));

    spans
}

/// render danmu message
pub fn render_danmu_message(
    msg: DanmuMsg,
    time: DateTime<Local>,
    render_type: bool,
) -> Line<'static> {
    let mut spans = render_basic_info(
        get_msg_type(render_type, MsgType::Danmu),
        msg.user,
        Some(time),
    );
    spans.push(Span::from(replace_emoji_to_unicode(&msg.content)));

    Line::from(spans)
}

/// render super chat message
pub fn render_super_chat_message(
    msg: SuperChatMsg,
    time: DateTime<Local>,
    render_type: bool,
) -> Line<'static> {
    let mut spans = render_basic_info(
        get_msg_type(render_type, MsgType::SuperChat),
        msg.user,
        Some(time),
    );
    spans.push(Span::from(format!("({} 元)", msg.price)));
    spans.push(Span::raw(" "));
    spans.push(Span::from(msg.content));

    Line::from(spans)
}

/// render gift message
pub fn render_gift_message(
    msg: GiftMsg,
    time: DateTime<Local>,
    render_type: bool,
) -> Line<'static> {
    let mut spans = render_basic_info(
        get_msg_type(render_type, MsgType::Gift),
        msg.user,
        Some(time),
    );
    spans.push(Span::from(format!(
        "赠送了{} * {} ",
        msg.gift_name, msg.amount
    )));

    let total = (msg.price * msg.amount) as f32 / 1000.0;
    if total > 0.0 {
        spans.push(
            Span::from(format!("({:.1} 元)", total))
                .fg(Color::LightMagenta)
                .bold(),
        );
    }

    if let Some(master) = msg.send_master {
        spans.push(Span::from(format!(" 给 {}", master.uname)));
    }

    Line::from(spans)
}

/// render guard buy message
pub fn render_guard_buy_message(
    msg: GuardBuyMsg,
    time: DateTime<Local>,
    render_type: bool,
) -> Line<'static> {
    let mut spans = render_basic_info(
        get_msg_type(render_type, MsgType::GuardBuy),
        msg.user,
        Some(time),
    );
    spans.push(Span::raw("在你的直播间购买了"));
    spans.push(Span::from(msg.gift_name).fg(tailwind::GREEN.c400).bold());
    spans.push(Span::raw(" "));
    spans.push(Span::from(format!("({} 元)", msg.price / 1000)));

    Line::from(spans)
}

/// render user action message
pub fn render_user_action_message(
    msg: UserActionMsg,
    time: DateTime<Local>,
    render_type: bool,
) -> Line<'static> {
    let mut spans = render_basic_info(
        get_msg_type(render_type, MsgType::UserAction),
        msg.user,
        Some(time),
    );
    spans.push({
        match msg.action.as_str() {
            "enter" => Span::from("进入你的直播间"),
            "follow" => Span::from("关注了你"),
            "share" => Span::from("分享了你的直播间"),
            "like" => Span::from("为你的直播间点赞"),
            _ => Span::from(""),
        }
    });

    Line::from(spans)
}

#[inline]
fn get_msg_type(render_type: bool, msg_type: MsgType) -> Option<MsgType> {
    if render_type {
        Some(msg_type)
    } else {
        None
    }
}
