import { Hono } from 'hono'
import { installSteamCMD, isSteamInstalled } from './utils/steam'
import { WORKING_PROCESS_KEY, WORKING_PROCESS_MAP } from './const'
import { run } from 'node:test'
import { $ } from 'execa'

const app = new Hono()
app.get('/', async c => {
  return c.text('hello world')
})
app.notFound(async c => {
  return c.text('Not found', 404)
})
app.get('/api/getSteamCMDinfo', async c => {
  return c.json({
    version: '1.0.0',
    isDirExists: isSteamInstalled()
  })
})
app.get('/api/installSteamCMD', async c => {
  if (WORKING_PROCESS_MAP.get(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)) {
    return c.json({ running: true })
  }
  // WORKING_PROCESS_MAP.set(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD, true)
  await installSteamCMD(true)
  return c.json({ success: true })
})

export default app
