import { Hono } from 'hono'
import { stream } from 'hono/streaming'
import { getSSH } from './ssh'
import { accessSync, constants } from 'node:fs'
import { getDedicatedServerPid, getDedicatedServerSaves } from './utils/commond'

const checkInstallSteamCMD = async (isLocal = false) => {
  const STEAM_CMD_PATH = '/root/steam_cmd_download'
  if (isLocal) {
    try {
      accessSync(`${STEAM_CMD_PATH}/steamcmd.sh`, constants.F_OK)
      return true
    } catch (e) {
      return false
    }
  } else {
    const ssh = await getSSH()
    const { stdout } = await ssh.execCommand(
      `[ -f ${STEAM_CMD_PATH}/steamcmd.sh ] && echo yes`
    )
    return !!stdout
  }
}

const checkInstallGameServer = async (isLocal = false) => {
  const STEAM_CMD_PATH = '/root/steam_cmd_download'
  const DST_SERVER_PATH = '/root/dst_server_download'
  const installInfo = {
    steamCMD: false,
    gameServer: false
  }
  if (isLocal) {
    try {
      accessSync(`${STEAM_CMD_PATH}/steamcmd.sh`, constants.F_OK)
      installInfo.steamCMD = true
    } catch (e) {
      installInfo.steamCMD = false
    }
    try {
      accessSync(`${DST_SERVER_PATH}/version.text`, constants.F_OK)
      installInfo.gameServer = true
    } catch (e) {
      installInfo.gameServer = false
    }
    return installInfo
  } else {
    const ssh = await getSSH()
    const [{ stdout: steamCMD }, { stdout: gameServer }] = await Promise.all([
      ssh.execCommand(`[ -f ${STEAM_CMD_PATH}/steamcmd.sh ] && echo yes`),
      ssh.execCommand(`[ -f ${DST_SERVER_PATH}/version.txt ] && echo yes`)
    ])
    installInfo.gameServer = !!gameServer
    installInfo.steamCMD = !!steamCMD

    return installInfo
  }
}

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
let runningInstallSteamCMD_STDERR: string[] = []
app.post('/remote/env/install/steamCMD', async c => {
  if (runningInstallSteamCMD) {
    return c.json({
      success: false,
      message: 'Another installation is running',
      stdout: runningInstallSteamCMD_STDOUT
    })
  }
  runningInstallSteamCMD = true
  const isInstalled = await checkInstallSteamCMD()

  if (isInstalled) {
    runningInstallSteamCMD = false
    return c.json({
      success: true,
      message: 'SteamCMD is already installed',
      stdout: runningInstallSteamCMD_STDOUT
    })
  }
  console.log('res', isInstalled)
  const ssh = await getSSH()
  await ssh.execCommand(`mkdir -p /root/steam_cmd_download`)
  runningInstallSteamCMD_STDOUT = []
  runningInstallSteamCMD_STDERR = []
  ssh.execCommand(
    `curl -sqkL https://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz | tar zxf - -C /root/steam_cmd_download`,
    {
      onStdout: chunk => {
        console.log('onStdout', chunk.toString())
        runningInstallSteamCMD_STDOUT.push(chunk.toString())
      },
      onStderr(chunk) {
        // runningInstallSteamCMD = false
        console.log('onStderr', chunk.toString())
        runningInstallSteamCMD_STDERR.push(chunk.toString())
      },
      onChannel: channel => {
        channel.on('exit', () => {
          runningInstallSteamCMD = false
          console.log('close channel')
        })
      }
    }
  )
  return c.json({ success: true, message: 'SteamCMD is installing' })
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

let runningInstallGameServer = false
const runningInstallGameServer_STDOUT: string[] = []
const runningInstallGameServer_STDERR: string[] = []
app.post('/remote/env/install/gameServer', async c => {
  const STEAM_CMD_PATH = '/root/steam_cmd_download'
  const DST_SERVER_PATH = '/root/dst_server_download'
  if (runningInstallGameServer) {
    return c.json({
      success: false,
      message: 'Another installation is running',
      stdout: runningInstallGameServer_STDOUT
    })
  }
  runningInstallGameServer = true
  const installInfo = await checkInstallGameServer()
  console.log('installInfo', installInfo)
  if (installInfo.gameServer && installInfo.steamCMD) {
    runningInstallGameServer = false
    return c.json({
      success: true,
      message: 'all is already installed',
      stdout: runningInstallGameServer_STDOUT
    })
  }

  ;(async () => {
    const ssh = await getSSH()
    if (!installInfo.steamCMD) {
      console.log('install steamCMD')
      await ssh.execCommand(`mkdir -p ${STEAM_CMD_PATH}`)
      await ssh.execCommand(
        `curl -sqkL https://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz | tar zxf - -C ${STEAM_CMD_PATH}`
      )
      console.log('steamCMD installed')
    }
    if (!installInfo.gameServer) {
      console.log('install gameServer')
      await ssh.execCommand(`chmod +x ${STEAM_CMD_PATH}/steamcmd.sh`)
      // /root/steam_cmd_download/steamcmd.sh +login anonymous +force_install_dir /root/dst_server_download +app_update 343050 validate +quit
      // 同时会在根目录下创建 Steam
      await ssh.execCommand(
        `${STEAM_CMD_PATH}/steamcmd.sh +force_install_dir ${DST_SERVER_PATH} +login anonymous +app_update 343050 validate +quit`,
        {
          onStdout: chunk => {
            console.log('onStdout', chunk.toString())
            runningInstallGameServer_STDOUT.push(chunk.toString())
          },
          onStderr(chunk) {
            console.log('onStderr', chunk.toString())
          }
        }
      )
      console.log('gameServer installed')
    }
  })()

  return c.json({
    success: true,
    data: installInfo,
    message: 'installing is installing'
  })
})

app.post('/remote/save/create', async c => {
  const ssh = await getSSH()
  const result = await ssh.putDirectory(
    '/Users/linchaoting/Developer/github/dst-server-admin/packages/server/static',
    '/root/transfer'
  )
  console.log('result', result)
  return c.json({ success: true })
})

app.post('/remote/server/start', async c => {
  const ssh = await getSSH()
  ssh.execCommand(
    // `ls`,
    `screen -dmS dst-server ./dontstarve_dedicated_server_nullrenderer_x64 -cluster 1 -shard world1`,
    {
      cwd: '/root/dst_server_download/bin64',
      onStdout: chunk => {
        console.log('onStdout-1', chunk.toString())
      },

      onStderr: chunk => {
        console.log('onStderr-1', chunk.toString())
      }
    }
  )
  ssh.execCommand(
    // `ls`,
    `./dontstarve_dedicated_server_nullrenderer_x64 -cluster 1 -shard world2`,
    {
      cwd: '/root/dst_server_download/bin64',
      onStdout: chunk => {
        console.log('onStdout-2', chunk.toString())
      },

      onStderr: chunk => {
        console.log('onStderr-2', chunk.toString())
      }
    }
  )
  return c.json({ success: true })
})

app.get('/remote/saves', async c => {
  const saves = await getDedicatedServerSaves()
  return c.json({ success: true, data: saves })
})

app.get('/remote/status/runningServer', async c => {
  const saves = await getDedicatedServerSaves()
  const allWorldsMap = saves.reduce((acc, cur) => {
    cur.worldNames.forEach(worldName => {
      acc.set(`${cur.saveName}/${worldName}`, '')
    })
    return acc
  }, new Map<string, string>())

  await Promise.all(
    Array.from(allWorldsMap.keys()).map(async path => {
      const [saveName, worldName] = path.split('/')
      const pid = await getDedicatedServerPid(saveName, worldName)
      allWorldsMap.set(path, pid)
    })
  )
  const result = saves.map(save => {
    return {
      saveName: save.saveName,
      worlds: save.worldNames.map(worldName => {
        const pid = allWorldsMap.get(`${save.saveName}/${worldName}`)
        return {
          worldName,
          pid
        }
      })
    }
  })
  return c.json({ success: true, data: result })
})

export default app
