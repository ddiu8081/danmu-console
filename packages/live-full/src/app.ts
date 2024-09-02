import { Tui, MsgType, restoreTerminal } from '@bilicli/bilicli-napi'
import { Message, MsgHandler, startListen } from 'blive-message-listener'
import open from 'open'
import { isAbsolute, join } from 'node:path'
import { homedir, userInfo } from 'node:os'
import { existsSync, mkdirSync, writeFileSync } from 'node:fs'
import { createRequire } from 'node:module'

export interface AppOptions {
  cookie?: string
  config?: string
  uid?: string
  badge: boolean
  rank: boolean
}

export interface EditOptions {
  config: string
}

export class App {
  private readonly roomId: number
  private readonly tui: Tui

  constructor(roomId: string | undefined | number, private options: AppOptions) {
    if (!roomId) {
      const config = getConfigPath(this.options.config!)

      if (!existsSync(config)) {
        throw new Error('房间号或配置文件路径不能为空')
      }

      try {
        const require = createRequire(join(config, '..'))
        const c = require(config.endsWith('.js') ? config : `${config}.js`)
        
        roomId = c.roomId
        this.options.cookie = c?.cookie || ''
        this.options.uid = c?.uid
      } catch (error) {
        console.error(error)
        throw new Error('配置文件格式不正确')
      }
    } else if(!this.options?.cookie || !this.options?.uid) {
      const config = getConfigPath(this.options.config!)
      if (existsSync(config)) {
        try {
          const require = createRequire(join(config, '..'))
          const c = require(config.endsWith('.js') ? config : `${config}.js`)
          if(!roomId && c.roomId) {
            roomId = c.roomId
          }
          if(!this.options.cookie && c.cookie) {
            this.options.cookie = c.cookie
          }
          if(!this.options.uid && c.uid) {
            this.options.uid = c.uid
          }
        } catch {
        }
      }
    }

    if(typeof roomId === 'number') {
      this.roomId = roomId
    } else {
      this.roomId = parseInt(roomId!)
    }


    if (isNaN(this.roomId)) {
      throw new Error('房间号格式不正确')
    }

    this.tui = new Tui(this.roomId, this.options?.cookie)

    const handler: MsgHandler = {
      onAttentionChange: ({ body }) => this.tui.sendAttentionChange(body.attention),
      onWatchedChange: ({ body }) => this.tui.sendWatcherChange(body.num),
      onLiveStart: () => this.tui.sendLiveChange(true),
      onLiveEnd: () => this.tui.sendLiveChange(false),
      onIncomeDanmu: (msg) => this.tui.sendMsg(MsgType.Danmu, JSON.stringify(this.mixTimestamp2Body(msg))),
      onIncomeSuperChat: (msg) =>  this.tui.sendMsg(MsgType.SuperChat, JSON.stringify(this.mixTimestamp2Body(msg))),
      onGift: (msg) => this.tui.sendMsg(MsgType.Gift, JSON.stringify(this.mixTimestamp2Body(msg))),
      onGuardBuy: (msg) => this.tui.sendMsg(MsgType.GuardBuy, JSON.stringify(this.mixTimestamp2Body(msg))),
      onUserAction: (msg) => this.tui.sendMsg(MsgType.UserAction, JSON.stringify(this.mixTimestamp2Body(msg))),
    }

    let uid: number | undefined
    if (this.options?.uid) {
      uid = parseInt(this.options.uid)
      if (isNaN(uid)) {
        throw new Error('UID 格式不正确')
      }
    }

    startListen(this.roomId, handler, {
      ws: {
        headers: {
          'Cookie': this.options?.cookie || "",
        },
        uid,
      },
    })
  }

  async run() {
    try {
      await this.tui.run()
    } catch {
      restoreTerminal()
    }
  }

  private mixTimestamp2Body(msg: Message<any>) {
    return {
      ...msg.body,
      timestamp: msg?.timestamp || Date.now(),
    }
  }
}

function getConfigPath(config: string) {
  let target = config
  let userinfo = userInfo()
  
  switch (process.platform) {
    case 'darwin':
      if(!isAbsolute(config)) {
        target = join(homedir(), userinfo.username, '.config', 'bilicli')
        if(!existsSync(target)) {
          mkdirSync(target, { recursive: true })
        }
        target = join(target, config)
      }
      break
    case 'win32':
      if(!isAbsolute(config)) {
        target = join(homedir(), userinfo.username, 'AppData', 'Roaming', 'bilicli')
        if(!existsSync(target)) {
          mkdirSync(target, { recursive: true })
        }
        target = join(target, config)
      }
      break
    default:
      if(!isAbsolute(config)) {
        target = join(homedir(), userinfo.username, '.config', 'bilicli')
        if(!existsSync(target)) {
          mkdirSync(target, { recursive: true })
        }
        target = join(target, config)
      }
  }

  return target
}

export async function openEditor(config: string) {
  let target = getConfigPath(config)

  if(!existsSync(target)) {
    writeFileSync(target, `
module.exports = {
  cookie: '',
  uid: undefined,
  // roomId: 1,
}
    `)
  }

  await open(target)
}
