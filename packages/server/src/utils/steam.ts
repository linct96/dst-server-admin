import { accessSync, mkdirSync, constants } from 'node:fs'
import {
  DST_SERVER_PATH,
  DST_UGC_MOD_PATH,
  STEAM_CMD_PATH,
  STEAM_DOWNLOAD_URL_LINUX
} from '../const'
import { $ } from 'execa'
import { rimrafSync } from 'rimraf'

export const isSteamInstalled = () => {
  try {
    accessSync(STEAM_CMD_PATH, constants.F_OK)
    return true
  } catch (e) {
    return false
  }
}

export const installENV = async () => {
  console.log('Installing environment...')
  const scriptPath = `./scripts/install-env.sh`
  await $`chmod +x ${scriptPath}`
  await $`${scriptPath}`
  console.log('Environment installed.')
}

export const installSteamCMD = async (forceInstall = false) => {
  if (forceInstall) {
    rimrafSync(STEAM_CMD_PATH)
  }
  if (!isSteamInstalled()) {
    mkdirSync(STEAM_CMD_PATH, { recursive: true })
    console.log('Installing SteamCMD...')
    const childProcess = $({
      shell: true,
      verbose: 'full'
    })`curl -sqkL ${STEAM_DOWNLOAD_URL_LINUX}`
      .pipe`tar zxf - -C ${STEAM_CMD_PATH}`
    // childProcess.stdout.pipe(process.stdout)
    await childProcess
    console.log('SteamCMD installed.')
  } else {
    console.log('SteamCMD is already installed.')
  }
}

export const installGameServer = async (forceInstall = false) => {
  if (forceInstall) {
    rimrafSync(DST_SERVER_PATH)
  }
  mkdirSync(DST_SERVER_PATH, { recursive: true })
  mkdirSync(DST_UGC_MOD_PATH, { recursive: true })
  console.log('Installing DST server...')
  const childProcess = $({
    shell: true,
    verbose: 'full'
  })`chmod +x ${STEAM_CMD_PATH}/steamcmd.sh && ${STEAM_CMD_PATH}/steamcmd.sh +login anonymous +force_install_dir ${DST_SERVER_PATH} +app_update 343050 validate +quit`
  await childProcess
  console.log('DST server installed.')
}
