import { createFileRoute } from '@tanstack/react-router'
import { Button, Space } from 'antd'
import { SERVER_URL } from '../const'
import { resolveReader } from '../utils'

export const Route = createFileRoute('/test')({
  component: Test
})

function Test() {
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
    const res = await fetch(`${SERVER_URL}/api/remote/env/install/node`, {
      method: 'POST'
    })
    if (res.ok && res.body) {
      const reader = res.body.getReader()
      const readerResult = resolveReader(reader, str => {
        console.log('stream', str)
      })
      console.log('readerResult', readerResult)
      // const result = await res.json()
      // console.log('initOS result', result)
    }
  }
  const installSteamCMD = async () => {
    const res = await fetch(`${SERVER_URL}/api/remote/env/install/steamCMD`, {
      method: 'POST'
    })
    if (res.ok) {
      const result = await res.json()
      console.log('installSteamCMD result', result)
    }
  }
  return (
    <div>
      <Space direction="vertical">
        <Button onClick={installSteamCMD}>安装steamCMD</Button>
        <Button onClick={installNode}>安装node</Button>
        <Button onClick={serverInit}>系统初始化</Button>
        <Button onClick={getGlobalVar}>get global var</Button>
        <Button onClick={init}>init</Button>
        <Button onClick={createSave}>create save</Button>
      </Space>
    </div>
  )
}
