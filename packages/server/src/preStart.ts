import {
  accessSync,
  constants,
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  statSync
} from 'fs'
import { DST_SAVE_PATH, DST_SERVER_PATH, STEAM_CMD_PATH } from './const'
import { GlobalVar } from './global'

export default async function preStart() {
  try {
    accessSync(STEAM_CMD_PATH, constants.F_OK)
    GlobalVar.isSteamCMDInstalled = true
  } catch (e) {
    GlobalVar.isSteamCMDInstalled = false
  }

  try {
    const version = readFileSync(`${DST_SERVER_PATH}/version.txt`)
      .toString()
      .trim()
    GlobalVar.dstServerVersion = version
  } catch (e) {
    GlobalVar.dstServerVersion = ''
  }

  try {
    statSync(`${DST_SAVE_PATH}`).isDirectory()
  } catch (e) {
    mkdirSync(DST_SAVE_PATH)
  }

  try {
    const count = readdirSync(`${DST_SAVE_PATH}`).filter(p =>
      statSync(`${DST_SAVE_PATH}/${p}`).isDirectory()
    ).length
    GlobalVar.saveCount = count
  } catch (e) {
    GlobalVar.saveCount = 0
  }
}
