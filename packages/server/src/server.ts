import { Hono } from 'hono'
import { stream } from 'hono/streaming'
import { NodeSSH } from 'node-ssh'

const ssh = new NodeSSH()

const getSSH = async () => {
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

const app = new Hono()
app.get('/initNode', async c => {
  const ssh = await getSSH()
  const res = await ssh.execCommand(
    'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -'
  )
  await ssh.execCommand('apt-get install nodejs -y')
  return c.json({ message: res.stdout })
})

app.get('/initOS', async c => {
  const ssh = await getSSH()
  const res = await ssh.execCommand(
    'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -'
  )
  await ssh.execCommand('apt-get install nodejs -y')
  return c.json({ message: res.stdout })
})

const remoteConfig = {
  host: '101.126.78.130',
  username: 'root',
  password: 'byte_id_rsa',
  port: 22,
  privateKeyPath: '/Users/linchaoting/.ssh/byte_id_rsa'
}
app.get('/server/config/remote', async c => {
  const ssh = await getSSH()
  const res = await ssh.execCommand(
    'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -'
  )
  await ssh.execCommand('apt-get install nodejs -y')
  return c.json({ message: res.stdout })
})

export default app
