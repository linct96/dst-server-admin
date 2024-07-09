import { createFileRoute } from '@tanstack/react-router'
import { Button, Space } from 'antd'
import { SERVER_URL } from '../const'

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
  return (
    <div>
      <Space direction="vertical">
        <Button onClick={serverInit}>系统初始化</Button>
        <Button onClick={getGlobalVar}>get global var</Button>
        <Button onClick={init}>init</Button>
        <Button onClick={createSave}>create save</Button>
      </Space>
    </div>
  )
}
