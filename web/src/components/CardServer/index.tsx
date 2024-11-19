import {
  Badge,
  Button,
  Card,
  Descriptions,
  Flex,
  Progress,
  ProgressProps,
  Space,
  Statistic,
  Table,
  TableColumnsType
} from 'antd'
import { SERVER_URL } from '../../const'
import { CardServerWrap } from './index.style'
import { useEffect, useState } from 'react'
import { useRequest, useInterval } from 'ahooks'

export default function CardServer() {
  const { data, loading, refresh } = useRequest(
    async () => {
      const response = await fetch(
        `${SERVER_URL}/api/auth/system/get_system_info`
      )
      const json = await response.json()
      return json.data
    },
    {
      pollingInterval: 1000 * 2
    }
  )

  // const data = state.value?.data
  // if (!data) return null
  // console.log('data', data)
  if (!data) return null
  console.log('data', data)
  const conicColors: ProgressProps['strokeColor'] = {
    '0%': '#52c41a',
    '50%': '#ffe58f',
    '100%': '#ff4d4f'
  }
  const getStrokeColor = (percent: number) => {
    if (percent < 50) {
      return '#52c41a'
    } else if (percent < 80) {
      return '#faad14'
    } else {
      return '#ff4d4f'
    }
  }
  return (
    <CardServerWrap className="card-server-wrap">
      <Card bordered={false} title="服务器状态" style={{ flex: 1 }}>
        <Flex gap="large" justify="space-between">
          <Flex gap="middle">
            <Progress
              size={68}
              type="circle"
              percent={data.cpu_usage}
              strokeColor={getStrokeColor(data.cpu_usage)}
            />
            <Statistic title="CPU使用" value={`核心：${data.cpu_count}`} />
          </Flex>

          <Flex gap="middle">
            <Progress
              size={68}
              type="circle"
              percent={data.memory_usage}
              strokeColor={getStrokeColor(data.memory_usage)}
            />
            <Statistic
              title="内存使用/总内存"
              value={`${data.memory_used}G/${data.memory_total}G`}
            />
          </Flex>

          <Flex gap="middle">
            <Progress
              size={68}
              type="circle"
              percent={data.disk_usage}
              strokeColor={getStrokeColor(data.disk_usage)}
            />
            <Statistic
              title="磁盘使用/总磁盘"
              value={`${data.disk_used}G/${data.disk_total}G`}
            />
          </Flex>
        </Flex>
      </Card>
      <Card bordered={false} title="服务器信息">
        <Descriptions
          size="small"
          column={1}
          items={[
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
          ]}
          style={{ width: '200px' }}
        />
      </Card>
    </CardServerWrap>
  )
}
