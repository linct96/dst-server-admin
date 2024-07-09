import { NodeSSH } from 'node-ssh'

const ssh = new NodeSSH()

export const getSSH = async () => {
  if (ssh.isConnected()) {
    return ssh
  } else {
    await ssh.connect({
      host: '101.126.78.130',
      username: 'root',
      privateKeyPath: '/Users/linchaoting/.ssh/byte_id_rsa'
    })
  }
  return ssh
}
