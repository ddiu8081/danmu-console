import { $fetch } from 'ofetch'

export interface RoomInfo {
  uid: number
  room_id: number
  short_id: number
  attention: number
  online: number
  is_portrait: boolean
  description: string
  live_status: number
  area_id: number
  parent_area_id: number
  parent_area_name: string
  background: string
  title: string
  user_cover: string
  keyframe: string
  live_time: string
  area_name: string
}

interface RoomInfoResponse {
  code: number
  msg: string
  message: string
  data: RoomInfo
}

export const getRoomInfo = async (roomId: number) => {
  const url = `https://api.live.bilibili.com/room/v1/Room/get_info?room_id=${roomId}`
  const response = await $fetch<RoomInfoResponse>(url)
  return response.code === 0 ? response.data : null
}
