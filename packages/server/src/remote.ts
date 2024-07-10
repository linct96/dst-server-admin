import { Hono } from 'hono'
import { stream } from 'hono/streaming'
import { getSSH } from './ssh'
import { stdout } from 'process'

const app = new Hono()

let running = false
app.post('/remote/env/install/node', async c => {
  if (running) {
    return c.json({
      success: false,
      message: 'Another installation is running'
    })
  }
  const ssh = await getSSH()
  const downloadResult = await ssh.execCommand(
    'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -'
  )
  console.log('curl success', downloadResult)
  const installResult = await ssh.execCommand('apt-get install nodejs -y')
  console.log('install success', installResult)
  running = false
  return c.json({ success: true })
  return stream(c, stream => {
    return new Promise(resolve => {
      ssh.execCommand(
        'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -',
        {
          onStdout: chunk => {
            console.log(chunk.toString('utf-8'))
            stream.write(chunk)
          },
          onChannel: channel => {
            channel.on('exit', () => {
              console.log('close channel')
              stream.close()
              resolve()
            })
          }
        }
      )
    })
  })
})

let runningInstallSteamCMD = false
let runningInstallSteamCMD_STDOUT: string[] = []
app.post('/remote/env/install/steamCMD', async c => {
  if (runningInstallSteamCMD) {
    return c.json({
      success: false,
      message: 'Another installation is running',
      stdout: runningInstallSteamCMD_STDOUT
    })
  }
  runningInstallSteamCMD = true
  const ssh = await getSSH()
  await ssh.execCommand(`mkdir -p /root/steam_cmd_download`)
  ssh.execCommand(
    `curl -sqkL https://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz | tar zxf - -C /root/steam_cmd_download`,
    {
      onStdout: chunk => {
        console.log(chunk.toString())
        runningInstallSteamCMD_STDOUT.push(chunk.toString())
      },
      onStderr(chunk) {
        console.log(chunk.toString())
        // runningInstallSteamCMD_STDOUT.push(chunk.toString())
      },
      onChannel: channel => {
        channel.on('exit', () => {
          runningInstallSteamCMD = false
          runningInstallSteamCMD_STDOUT = []
          console.log('close channel')
        })
      }
    }
  )
  // console.log('curl success', downloadResult)
  // const installResult = await ssh.execCommand('apt-get install nodejs -y')
  // console.log('install success', installResult)
  return c.json({ success: true })
  return stream(c, stream => {
    return new Promise(resolve => {
      ssh.execCommand(
        'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -',
        {
          onStdout: chunk => {
            console.log(chunk.toString('utf-8'))
            stream.write(chunk)
          },
          onChannel: channel => {
            channel.on('exit', () => {
              console.log('close channel')
              stream.close()
              resolve()
            })
          }
        }
      )
    })
  })
})

export default app
