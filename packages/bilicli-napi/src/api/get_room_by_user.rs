use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct WearedV2 {
    pub(crate) name: String,
    pub(crate) level: u32,
    pub(crate) guard_level: u32,
    pub(crate) v2_medal_color_text: String,
    pub(crate) v2_medal_color_level: String,
}

#[derive(Deserialize)]
pub struct Medal {
    pub(crate) curr_weared_v2: WearedV2,
}

#[derive(Deserialize)]
pub struct RoomUserInfo {
    pub medal: Medal,
}

#[derive(Deserialize)]
struct GetRoomByUserResponse {
    code: u32,
    message: String,
    data: RoomUserInfo,
}

pub async fn get_room_by_user(room_id: u32) -> Result<RoomUserInfo, reqwest::Error> {
    let resp = reqwest::get(format!(
        "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByUser?room_id={}",
        room_id
    ))
    .await?
    .json::<GetRoomByUserResponse>()
    .await?;

    Ok(resp.data)
}
