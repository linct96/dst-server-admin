import { createLazyFileRoute, Link } from '@tanstack/react-router'
import { Button, Space } from 'antd'

export const Route = createLazyFileRoute('/initialize/mode')({
  component: Mode
})

function Mode() {
  return (
    <div>
      <Space>
        <Button>本地创建</Button>
        <Button>云服务器创建</Button>
      </Space>
    </div>
  )
}
