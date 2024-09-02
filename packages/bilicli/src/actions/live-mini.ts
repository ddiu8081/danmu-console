import { startApp } from '@bilicli/live-mini'

interface ArgOptions {
  badge: boolean
  rank: boolean
}

const liveMini = async (roomId: string, options: ArgOptions) => {
  await startApp(~~roomId, {
    badge: options.badge,
    rank: options.rank,
  })
}

export { liveMini }
