use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct RoomInfo {
    pub uid: u64,
    pub room_id: u32,
    pub short_id: u32,
    pub attention: u32,
    pub online: u32,
    pub is_portrait: bool,
    pub description: String,
    pub live_status: u32,
    pub area_id: u32,
    pub parent_area_id: u32,
    pub parent_area_name: String,
    pub background: String,
    pub title: String,
    pub user_cover: String,
    pub keyframe: String,
    pub live_time: String,
    pub area_name: String,
}

#[derive(Deserialize)]
pub struct GetRoomInfoResponse {
    pub code: i32,
    pub message: String,
    pub data: RoomInfo,
}

pub async fn get_room_info(room_id: u32) -> Result<RoomInfo, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(
        "https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}",
        room_id
    ))
    .await?;

    let response = response.json::<GetRoomInfoResponse>().await?;

    if response.code != 0 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            response.message,
        )))?
    }

    Ok(response.data)
}
