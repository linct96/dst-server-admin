import { ResultPromise } from 'execa'

export const enum WORKING_PROCESS_KEY {
  INSTALL_STEAM_CMD = 1
}
export const WORKING_PROCESS_MAP = new Map<WORKING_PROCESS_KEY, ResultPromise>()

export const HOME_PATH = '${HOME}'
export const STEAM_CMD_PATH = `${HOME_PATH}/STEAM_CMD`
export const DST_SERVER_PATH = `${HOME_PATH}/DST_SERVER`
export const DST_SAVE_PATH = `${HOME_PATH}/DST_SERVER_SAVE`
