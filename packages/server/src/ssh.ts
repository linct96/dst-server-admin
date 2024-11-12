import { NodeSSH } from 'node-ssh'
import { homedir } from 'node:os'
import { resolve } from 'node:path'

const ssh = new NodeSSH()

const sshMap = new Map<string, NodeSSH>()

interface SSHOptions {
  host: string
  username: string
  port?: number
  password?: string
  privateKey?: string
}
export const loginSSH = async (options: SSHOptions) => {
  const instance = getSSHInstance(options.host)
  if (instance) return instance
  const ssh = new NodeSSH()
  await ssh.connect({
    host: '101.126.78.130',
    username: 'root',
    // privateKeyPath: '/Users/linchaoting/.ssh/byte_id_rsa'
    privateKeyPath: resolve(homedir(), '.ssh', 'byte_id_rsa')
  })

  sshMap.set(options.host, ssh)
  return ssh
}
export const getSSHInstance = (ip: string) => {
  const instance = sshMap.get(ip)
  if (!instance) return
  if (!instance.isConnected()) {
    sshMap.delete(ip)
    return
  }
  return instance
}
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
