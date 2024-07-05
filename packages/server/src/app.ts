import { Hono } from 'hono'
import { cors } from 'hono/cors'
import {
  installENV,
  installGameServer,
  installSteamCMD,
  isSteamInstalled
} from './utils/steam'
import { $ } from 'execa'
import { GlobalVar } from './global'
import { mkdirSync, writeFileSync } from 'fs'
import { stringify } from 'ini'
import { DST_SAVE_PATH } from './const'
import { createCluster } from './utils/create'
const sleep = (time: number) => {
  return new Promise(resolve => {
    setTimeout(resolve, time)
  })
}
const app = new Hono()
app.use('/api/*', cors())

app.get('/', async c => {
  return c.text('hello world')
})

app.notFound(async c => {
  return c.text('Not found', 404)
})

app.get('/api/init', async c => {
  if (GlobalVar.initializing) return c.json({ initializing: true })
  GlobalVar.initializing = true
  if (!GlobalVar.isSteamCMDInstalled) {
    try {
      console.log('Installing SteamCMD')
      await installSteamCMD(true)
      GlobalVar.isSteamCMDInstalled = true
      console.log('SteamCMD installed')
    } catch (e) {
      console.error(e)
      GlobalVar.isSteamCMDInstalled = false
      GlobalVar.initializing = false
      return c.json({ error: e }, 500)
    }
  }
  if (GlobalVar.dstServerVersion === '') {
    console.log('Installing DST Server')
    try {
      await installGameServer(true)
    } catch (e) {
      console.error(e)
      GlobalVar.dstServerVersion = ''
      GlobalVar.initializing = false
      return c.json({ error: e }, 500)
    }
  }
  GlobalVar.initializing = false
  return c.json(GlobalVar)
  // return c.json(GlobalVar)
})

app.get('/api/getGlobalVar', async c => {
  return c.json(GlobalVar)
})

app.get('/api/createSave', async c => {
  await createCluster()
  return c.json(GlobalVar)
})

app.get('/api/getSteamCMDinfo', async c => {
  return c.json({
    version: '1.0.0',
    isDirExists: isSteamInstalled()
  })
})

app.get('/api/installSteamCMD', async c => {
  if (GlobalVar.runningInstallSteamCMD) return c.json({ running: true })
  GlobalVar.runningInstallSteamCMD = true
  await installSteamCMD(true)
  GlobalVar.runningInstallSteamCMD = false
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
