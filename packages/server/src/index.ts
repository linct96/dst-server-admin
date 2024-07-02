import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import axios from 'axios'
import { createWriteStream } from 'fs'
import { $, execa } from 'execa'
import { WORKING_PROCESS_KEY, WORKING_PROCESS_MAP } from './const'
import terminate from 'terminate/promise'

const app = new Hono()

app.get('/', async c => {
  // const writer = createWriteStream('./a.tar.gz');

  // axios.get("https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz",{
  //   responseType:"stream"
  // }).then(res=>{
  //   res.data.pipe(writer)
  // })
  // for await (const line of $({ shell: true })`echo $\{HOME\}`) {
  //   console.warn(line)
  // }
  if (WORKING_PROCESS_MAP.get(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)) {
    return c.text('Progress is already in progress')
  }
  const process = $({ shell: true })`ping cip.cc`
  if (process.pid) {
    WORKING_PROCESS_MAP.set(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD, process)
  }
  try {
    ;(async () => {
      for await (const line of process) {
        console.warn(line)
      }
    })()
  } catch (error) {
    console.error('eeeeeee', error)
  }

  // try {
  //   if (progress) {
  //     return c.text('Progress is already in progress')
  //   }
  //   progress = true
  //   for await (const line of $({ shell: true })`ping cip.cc`) {
  //     console.warn(line)
  //   }
  // } catch (e) {
  //   console.error(e)
  // }

  // execa('echo ${HOME}', { shell: true }).then(result => {
  //   console.log(result.stdout)
  // })

  // execa('ping cip.cc', { shell: true }).then(result => {
  //   console.log(result.stdout)
  // })

  // await $`curl -qL https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz`

  return c.text('Hello Hono!')
})
app.get('/stop', async c => {
  const process = WORKING_PROCESS_MAP.get(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)
  if (process && process.pid) {
    console.log('Terminating process')
    await terminate(process.pid).catch(err => {
      console.error('eeeeee', err)
      return c.text('error terminating process' + err)
    })
    WORKING_PROCESS_MAP.delete(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)
    return c.text('Process terminated successfully')
  } else {
    return c.text('No process is running')
  }
})

const port = 3000
console.log(`Server is running on port ${port}`)

serve({
  fetch: app.fetch,
  port
})
