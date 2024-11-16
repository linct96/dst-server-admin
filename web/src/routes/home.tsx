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
    console.log(data)
  }
  useEffect(() => {
    init()
  }, [])

  const items: DescriptionsProps['items'] = [
    {
      key: '1',
      label: '系统版本',
      children: 'linux'
    },
    {
      key: '2',
      label: '服务器地址',
      children: '1810000000'
    }
  ]
  return (
    <div className="p-2">
      <Flex gap="large">
        <Card bordered={false} title="服务器状态" style={{ flex: 1 }}>
          <Flex gap="large" justify="space-between">
            <Flex gap="middle">
              <Progress size={68} type="circle" percent={(0.2 / 4) * 100} />
              <Statistic title="内存使用/总内存" value={`${0.2}G/${4}G`} />
            </Flex>

            <Flex gap="middle">
              <Progress size={68} type="circle" percent={(0.2 / 4) * 100} />
              <Statistic title="CPU使用" value={`${0.2}G/${4}G`} />
            </Flex>
            <Flex gap="middle">
              <Progress size={68} type="circle" percent={(0.2 / 4) * 100} />
              <Statistic title="磁盘使用/总磁盘" value={`${0.2}G/${4}G`} />
            </Flex>
          </Flex>
        </Card>
        <Card bordered={false} title="服务器信息">
          <Descriptions
            size="small"
            column={1}
            items={items}
            style={{ width: '200px' }}
          />
        </Card>
      </Flex>
      <Flex gap="large" style={{ marginTop: '16px' }}>
        <div style={{ flex: 1 }}>
          <Card
            bordered={false}
            title="当前世界"
            extra={
              <Space>
                <Button type="primary">重启</Button>
                <Button danger>停止</Button>
              </Space>
            }
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
              style={{ width: '200px' }}
            />
          </Card>
          <Card bordered={false} title="存档信息" style={{ marginTop: '16px' }}>
            2
          </Card>
        </div>
        <div style={{ flex: 2 }}>
          <Card bordered={false} title="服务器连接">
            2
          </Card>
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
