import { stringify } from 'ini'
import { mkdirSync, writeFileSync } from 'node:fs'
import { DST_SAVE_PATH } from '../const'

const getClusterConfig = () => {
  const clusterConfig = {
    STEAM: {
      steam_group_id: 0,
      steam_group_only: false,
      steam_group_admins: false
    },
    GAMEPLAY: {
      game_mode: 'endless',
      max_players: 4,
      pause_when_empty: true,
      vote_enabled: true
    },
    NETWORK: {
      cluster_intention: 'cooperative',
      cluster_name: 'Cluster' + Date.now(),
      cluster_description: '一起享受饥荒吧！',
      cluster_password: 'password',
      cluster_language: 'zh',
      autosaver_enabled: true,
      enable_vote_kick: true,
      tick_rate: 15,
      connection_timeout: 5000
    },
    MISC: {
      max_snapshots: 20,
      console_enabled: true
    },
    SHARD: {
      cluster_key: 'randomsecretkey',
      shard_enabled: true,
      bind_ip: '0.0.0.0',
      master_ip: '127.0.0.1',
      master_port: 11111
    }
  }
  return clusterConfig
}

export const createCluster = async (saveName?: string) => {
  const clusterConfig = getClusterConfig()
  console.log('clusterConfig', clusterConfig)
  if (saveName) {
    clusterConfig.NETWORK.cluster_name = saveName
  }
  const _saveName = clusterConfig.NETWORK.cluster_name
  const savePath = `${DST_SAVE_PATH}/${_saveName}`
  mkdirSync(`${savePath}`)
  writeFileSync(
    `${savePath}/cluster.ini`,
    stringify(clusterConfig, {
      whitespace: true,
      sort: true
    })
  )
  writeFileSync(`${savePath}/adminlist.txt`, '')
  writeFileSync(`${savePath}/blocklist.txt`, '')
  writeFileSync(`${savePath}/cluster_token.txt`, '')
  writeFileSync(`${savePath}/whitelist.txt`, '')
}
