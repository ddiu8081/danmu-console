import { openEditor } from '@bilicli/live-full'

export interface EditOptions {
  config: string
}

export function editConfigFile(options: EditOptions) {
  openEditor(options.config)
}
