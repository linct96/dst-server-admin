import { getSSH } from '../ssh'

const DST_SAVE_PATH = '/root/.klei/DoNotStarveTogether'
export const getDedicatedServerSaves = async () => {
  const ssh = await getSSH()
  const { stdout } = await ssh.execCommand(
    `find ${DST_SAVE_PATH} -type f -name cluster.ini -exec dirname {} \\; | xargs -I {} basename {}`
  )
  const saves = stdout.split('\n').filter(save => !!save)

  const getWorldNamesPromiseArr = saves.map(save =>
    ssh.execCommand(
      `find ${DST_SAVE_PATH}/${save} -type f -name server.ini -exec dirname {} \\; | xargs -I {} basename {}`
    )
  )
  const worldNamesArr = await Promise.all(getWorldNamesPromiseArr)
  const result = saves.map((save, index) => {
    return {
      saveName: save,
      worldNames: worldNamesArr[index].stdout
        .split('\n')
        .filter(worldName => !!worldName)
    }
  })
  return result
}

export const getDedicatedServerPid = async (
  saveName: string,
  worldName: string
) => {
  const ssh = await getSSH()
  const { stdout } = await ssh.execCommand(
    `ps -ef | grep -v grep | grep -v tail | grep dontstarve_dedicated_server | grep ${saveName} | grep ${worldName} | sed -n '1P' | awk '{print $2}'`
  )
  return stdout.trim()
}
