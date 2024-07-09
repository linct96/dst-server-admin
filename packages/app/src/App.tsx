import { useState } from 'react'
import './App.css'
import { Button, Input } from 'antd'
import { SERVER_URL } from './const'

const resolveReader = async (
  reader: ReadableStreamDefaultReader<Uint8Array>,
  onData?: (text: string) => void,
  onEnd?: () => void
) => {
  let isEnd = false
  const result = []
  while (!isEnd) {
    const { done, value } = await reader.read()
    if (done) {
      isEnd = true
      onEnd?.()
    } else {
      const text = new TextDecoder().decode(value)
      result.push(text)
      onData?.(text)
    }
  }
  return result
}

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

  const installNode = async () => {
    const res = await fetch(`${SERVER_URL}/api/remote/env/node`, {
      method: 'POST'
    })
    if (res.ok && res.body) {
      const reader = res.body.getReader()
      const readerResult = await resolveReader(reader)
      const result = await res.json()
      console.log('initOS result', result)
      console.log('readerResult', readerResult)
    }
  }
  return (
    <div>
      <Input placeholder="密钥文件路径" value={passwordPath} />
      <Input
        placeholder="密码"
        onChange={v => setPasswordPath(v.target.value.trim())}
      />
      <Button onClick={installNode}>安装node</Button>
      <Button onClick={serverInit}>系统初始化</Button>
      <Button onClick={getGlobalVar}>get global var</Button>
      <Button onClick={init}>init</Button>
      <Button onClick={createSave}>create save</Button>
    </div>
  )
}

export default App
