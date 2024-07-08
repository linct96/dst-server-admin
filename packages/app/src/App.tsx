import { useState } from 'react'
import './App.css'
import { Button, Input } from 'antd'
import { SERVER_URL } from './const'

function App() {
  const [count, setCount] = useState(0)
  const [passwordPath, setPasswordPath] = useState('/root/.dst-server-admin')
  const [password, setPassword] = useState('password')
  const getGlobalVar = async () => {
    const res = await fetch(`${SERVER_URL}/api/getGlobalVar`)
    console.log('getGlobalVar', res)
    if (res.ok) {
      const result = await res.json()
      console.log('getGlobalVar result', result)
    }
  }
  const init = () => {
    fetch(`${SERVER_URL}/api/init`)
  }
  const createSave = async () => {
    const res = await fetch(`${SERVER_URL}/api/createSave`)
    console.log('createSave', res)
    if (res.ok) {
      const result = await res.json()
      console.log('createSave result', result)
    }
  }
  const serverInit = async () => {
    const res = await fetch(`${SERVER_URL}/api/initOS`)
    console.log('initOS', res)
    if (res.ok) {
      const result = await res.json()
      console.log('initOS result', result)
    }
  }
  return (
    <div>
      <Input placeholder="密钥文件路径" value={passwordPath} />
      <Input
        placeholder="密码"
        onChange={v => setPasswordPath(v.target.value.trim())}
      />
      <Button onClick={serverInit}>系统初始化</Button>
      <Button onClick={getGlobalVar}>get global var</Button>
      <Button onClick={init}>init</Button>
      <Button onClick={createSave}>create save</Button>
    </div>
  )
}

export default App
