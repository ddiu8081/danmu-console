import { startInstance } from './instance'
import { getRoomInfo } from './utils/getInfo'
import { roomInfoCom } from './component/roomInfo'

export interface AppOptions {
  badge: boolean
  rank: boolean
}

const startApp = async (roomId: number, options: AppOptions) => {
  const roomInfo = await getRoomInfo(roomId)
  if (!roomInfo) {
    console.log('房间不存在')
    return process.exit(1)
  }
  // Room Info
  console.log(roomInfoCom(roomInfo))
  startInstance(roomInfo.room_id, options)
}

export { startApp }
