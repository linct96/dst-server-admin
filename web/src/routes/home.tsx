import { createFileRoute } from '@tanstack/react-router'
import {
  Button,
  Card,
  Descriptions,
  DescriptionsProps,
  Divider,
  Flex,
  Form,
  FormProps,
  Input,
  InputNumber,
  Progress,
  Radio,
  Space,
  Statistic
} from 'antd'
import { SERVER_URL } from '../const'
import { useEffect } from 'react'
import CardSaves from '../components/CardSaves'
import CardServer from '../components/CardServer'

type FieldType = {
  host?: string
  connectType: 'password' | 'key'
  password?: string
  key?: string
  port?: number
}
const initialValues: Partial<FieldType> = {
  host: '101.126.78.130',
  password: '',
  key: `-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACAJToOQSukYaRukLreyGAv6K6VGaZrQ5Kb5lEP87fgzOwAAAJiKK32qiit9
qgAAAAtzc2gtZWQyNTUxOQAAACAJToOQSukYaRukLreyGAv6K6VGaZrQ5Kb5lEP87fgzOw
AAAEARlDZvdYgyN5mkU2kPqvIFUo9GQydk/o0GmbxYD+f69glOg5BK6RhpG6Qut7IYC/or
pUZpmtDkpvmUQ/zt+DM7AAAAFWxpbmNoYW90aW5nQE1CUC5sb2NhbA==
-----END OPENSSH PRIVATE KEY-----
`,
  connectType: 'key'
}
function Home() {
  // return null
  const [form] = Form.useForm<FieldType>()

  const init = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/get_game_info`, {
      method: 'GET'
      // body: JSON.stringify({})
    })
    const data = await res.json()
    handleGetSystemInfo()
    // console.log(data)
  }

  const handleGetSystemInfo = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/get_system_info`, {
      method: 'GET'
      // body: JSON.stringify({})
    })
    const data = await res.json()
  }

  const handleUpdateGame = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/update_dst_server`, {
      method: 'POST',
      body: JSON.stringify({})
    })
    const data = await res.json()
    console.log(data)
  }
  const handleRefreshDstServer = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/get_game_info`, {
      method: 'GET'
      // body: JSON.stringify({})
    })
    const json = await res.json()
    console.log(json.data)
  }
  const handleStartDstServer = async (data: any) => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/start_dst_server`, {
      method: 'POST',
      body: JSON.stringify(data)
    })
    const json = await res.json()
    console.log(json.data)
  }
  const handleForceInstall = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/test_fn`, {
      method: 'GET'
      // body: JSON.stringify({})
    })
    // const data = await res.json()
  }
  const handleGetAllSaves = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/get_all_saves`, {
      method: 'GET'
      // body: JSON.stringify({})
    })
    const data = await res.json()
    console.log(data)
  }
  useEffect(() => {
    // init()
  }, [])

  return (
    <div className="p-2">
      <CardServer />
      <Flex gap="large" style={{ marginTop: '16px' }}>
        <div style={{ flex: 1 }}>
          <Card
            bordered={false}
            title="饥荒服务器信息"
            extra={
              <Space>
                <Button onClick={handleForceInstall}>安装</Button>
              </Space>
            }
          >
            <Descriptions
              size="small"
              column={1}
              items={[
                {
                  key: '1',
                  label: '游戏版本',
                  children: (
                    <Space>
                      <span>1.16.5</span>
                      <a onClick={handleUpdateGame}>更新游戏</a>
                    </Space>
                  )
                },
                {
                  key: '2',
                  label: '模式',
                  children: '无尽模式'
                }
              ]}
              style={{ width: '320px' }}
            />
          </Card>
          <Card
            bordered={false}
            title="当前世界"
            extra={
              <Space>
                <Button type="primary" onClick={handleStartDstServer}>
                  启动
                </Button>
                <Button danger>停止</Button>
              </Space>
            }
            style={{ marginTop: '16px' }}
          >
            <Descriptions
              size="small"
              column={1}
              items={[
                {
                  key: '1',
                  label: '房间名称',
                  children: '房间名称'
                },
                {
                  key: '2',
                  label: '模式',
                  children: '无尽模式'
                },
                {
                  key: '3',
                  label: '天数',
                  children: '100天'
                },
                {
                  key: '4',
                  label: '人数',
                  children: '2/9'
                }
              ]}
              style={{ width: '320px' }}
            />
          </Card>
        </div>
        <div style={{ flex: 2 }}>
          <CardSaves />
          <Card
            bordered={false}
            title="服务器连接"
            style={{ marginTop: '16px' }}
          >
            2332
          </Card>
        </div>
      </Flex>
    </div>
  )
}

export const Route = createFileRoute('/home')({
  component: Home
})
