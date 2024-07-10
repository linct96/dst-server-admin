import { NodeSSH } from 'node-ssh'
import { homedir } from 'node:os'
import { resolve } from 'node:path'

const ssh = new NodeSSH()

export const getSSH = async () => {
  if (ssh.isConnected()) {
    return ssh
  } else {
    console.log('connecting to ssh', homedir())
    await ssh.connect({
      host: '101.126.78.130',
      username: 'root',
      // privateKeyPath: '/Users/linchaoting/.ssh/byte_id_rsa'
      privateKeyPath: resolve(homedir(), '.ssh', 'byte_id_rsa')
    })
  }
  return ssh
}
