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
  const installGameServer = async () => {
    const res = await fetch(`${SERVER_URL}/api/remote/env/install/gameServer`, {
      method: 'POST'
    })
    if (res.ok) {
      const result = await res.json()
      console.log('installGameServer result', result)
    }
  }
  const putFiles = async () => {
    const res = await fetch(`${SERVER_URL}/api/remote/save/create`, {
      method: 'POST'
    })
    if (res.ok) {
      const result = await res.json()
      console.log('putFiles result', result)
    }
  }
  const startServer = async () => {
    const res = await fetch(`${SERVER_URL}/api/remote/server/start`, {
      method: 'POST'
    })
    if (res.ok) {
      const result = await res.json()
      console.log('startServer result', result)
    }
  }
  const getSaves = async () => {
    const res = await fetch(`${SERVER_URL}/api/remote/status/runningServer`, {
      method: 'GET'
    })
    if (res.ok) {
      const result = await res.json()
      console.log('getSaves result', result)
    }
  }
  return (
    <div>
      <Space direction="vertical">
        <Button onClick={getSaves}>getSaves</Button>
        <Button onClick={startServer}>start server</Button>
        <Button onClick={putFiles}>put files</Button>
        <Button onClick={installGameServer}>installGameServer</Button>
        <Button onClick={installNode}>安装node</Button>
        <Button onClick={serverInit}>系统初始化</Button>
        <Button onClick={getGlobalVar}>get global var</Button>
        <Button onClick={init}>init</Button>
        <Button onClick={createSave}>create save</Button>
      </Space>
    </div>
  )
}
