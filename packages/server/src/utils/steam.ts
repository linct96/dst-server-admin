import { accessSync, mkdirSync, constants } from 'node:fs'
import { STEAM_CMD_PATH, STEAM_DOWNLOAD_URL_LINUX } from '../const'
import { $ } from 'execa'
import { rimraf, rimrafSync, native, nativeSync } from 'rimraf'

export const isSteamInstalled = () => {
  try {
    accessSync(STEAM_CMD_PATH, constants.F_OK)
    return true
  } catch (e) {
    return false
  }
}

export const installSteamCMD = async (forceInstall = false) => {
  if (forceInstall) {
    rimrafSync(STEAM_CMD_PATH)
  }
  if (!isSteamInstalled()) {
    mkdirSync(STEAM_CMD_PATH, { recursive: true })
    console.log('Installing SteamCMD...')
    const childProcess = $({
      shell: true
    })`curl -sqL ${STEAM_DOWNLOAD_URL_LINUX} | tar zxf - -C ${STEAM_CMD_PATH}`
    // childProcess.stdout.pipe(process.stdout)
    await childProcess
  } else {
    console.log('SteamCMD is already installed.')
  }
}
