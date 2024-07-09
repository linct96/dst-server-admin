import { Hono } from 'hono'
import { stream } from 'hono/streaming'
import { NodeSSH } from 'node-ssh'

let ssh: NodeSSH | undefined

const app = new Hono()
app.get('/initOS', async c => {
  const ssh = new NodeSSH()

  await ssh.connect({
    host: '101.126.78.130',
    username: 'root',
    privateKeyPath: '/Users/linchaoting/.ssh/byte_id_rsa'
  })

  const res = await ssh.execCommand(
    'curl -sL https://deb.nodesource.com/setup_22.x | sudo -E bash -'
  )
  console.log('connect', ssh.isConnected())
  ssh.dispose()
  console.log('connect', ssh.isConnected())
  return c.json({ message: res.stdout })
})

export default app
