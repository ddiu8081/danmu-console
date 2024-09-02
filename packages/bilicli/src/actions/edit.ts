import { openEditor } from '@bilicli/live-full'

export interface EditOptions {
  config: string
}

export function edit(options: EditOptions) {
  openEditor(options.config)
}
