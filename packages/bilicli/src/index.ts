#!/bin/env node
import { name, version } from '../package.json'
import { cac } from 'cac'

import { liveFull } from './actions/live-full'
import { liveMini } from './actions/live-mini'
import { edit } from './actions/edit'

const cli = cac(name)

cli
  .command('live <room_id>', 'Open live console in mini mode')
  .option('--no-badge', 'Hide badge of user')
  .option('--no-rank', 'Hide rank of user')
  .action(liveMini)

cli
  .command('live-full <room_id>', 'Open live console in full mode')
  .option('--no-badge', 'Hide badge of user')
  .option('--no-rank', 'Hide rank of user')
  .option('--cookie <cookie>', 'Your bilibili cookie')
  .option('--uid <uid>', 'Your bilibili uid, and you can get it from https://space.bilibili.com')
  .option('--config [config]', 'Cli config file path', { default: 'bilicli.config.js' })
  .action(liveFull)

  cli
  .command('edit', 'Open editor to edit cli config file')
  .option('--config [config]', 'Custom config file path', { default: 'bilicli.config.js' })
  .action(edit)

cli.help()
cli.version(version)

cli.parse()
