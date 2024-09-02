#!/bin/env node
import { name, version } from '../package.json'
import { cac } from 'cac'

import { liveFull } from './actions/live-full'
import { liveMini } from './actions/live-mini'

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
  .action(liveFull)

cli.help()
cli.version(version)

cli.parse()
