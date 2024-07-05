import { Hono } from 'hono'
import { cors } from 'hono/cors'
import {
  installENV,
  installGameServer,
  installSteamCMD,
  isSteamInstalled
} from './utils/steam'
import { WORKING_PROCESS_KEY, WORKING_PROCESS_MAP } from './const'

const app = new Hono()
app.use('/api/*', cors())

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
  await installSteamCMD(true)
  return c.json({ running: false })
})

app.get('/api/installGameServer', async c => {
  await installGameServer(true)
  return c.json({ running: false })
})

app.get('/api/installSystemLibraries', async c => {
  await installENV()
  return c.json({ running: false })
})

export default app
