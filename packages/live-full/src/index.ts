import { App, AppOptions, openEditor } from "./app"

const startApp = async (roomId: number, options: AppOptions) => {
  const app = new App(roomId, options)
  await app.run()
  process.exit(0)
}

export { startApp, openEditor }
