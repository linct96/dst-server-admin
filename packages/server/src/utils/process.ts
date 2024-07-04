import { $ } from 'execa'
import {
  STEAM_CMD_PATH,
  STEAM_DOWNLOAD_URL_LINUX,
  WORKING_PROCESS_KEY,
  WORKING_PROCESS_MAP
} from '../const'

export const spawnProcessInstallSteamCMD = () => {
  if (WORKING_PROCESS_MAP.get(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)) {
    return true
  }
  const childProcess = $({
    shell: true
  })`curl cip.cc`
  childProcess.connected
  // const childProcess = $({
  //   shell: true
  // })`curl -sqL ${STEAM_DOWNLOAD_URL_LINUX} | tar zxf - -C ${STEAM_CMD_PATH}`
  const pid = childProcess.pid
  if (pid) {
    WORKING_PROCESS_MAP.set(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD, childProcess)
    return true
  }
  return false
}
