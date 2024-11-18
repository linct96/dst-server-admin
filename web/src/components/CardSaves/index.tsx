import { Badge, Button, Card, Space, Table, TableColumnsType } from 'antd'
import { SERVER_URL } from '../../const'
import { CardSavesWrap } from './index.style'
import { useEffect, useState } from 'react'
type ExpandedDataType = {
  key: React.Key
  date: string
  name: string
  upgradeNum: string
  world_name: string
}
// save_name: String,
//     cluster_name: String,
//     cluster_description: String,
//     cluster_password: String,
//     game_mode: String,
//     max_players: String,
//     pvp: String,
type DataType = {
  key: React.Key
  save_name: string
  cluster_name: string
  cluster_description: string
  cluster_password: string
  game_mode: string
  max_players: string
  pvp: string
  worlds: ExpandedDataType[]
}
export default function CardSaves() {
  const [dataSource, setDataSource] = useState<DataType[]>([])
  const init = async () => {
    await handleGetAllSaves()
  }

  const handleGetAllSaves = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/system/get_all_saves`, {
      method: 'GET'
      // body: JSON.stringify({})
    })
    const json = await res.json()
    console.log(json)
    setDataSource(json.data)
  }
  useEffect(() => {
    init()
  }, [])

  const expandColumns: TableColumnsType<ExpandedDataType> = [
    { title: 'world_name', dataIndex: 'world_name' },
    { title: 'Date', dataIndex: 'date', key: 'date' },
    { title: 'Name', dataIndex: 'name', key: 'name' },
    {
      title: 'Status',
      key: 'state',
      render: () => <Badge status="success" text="Finished" />
    },
    { title: 'Upgrade Status', dataIndex: 'upgradeNum', key: 'upgradeNum' },
    {
      title: 'Action',
      key: 'operation',
      render: () => (
        <Space size="middle">
          <a>启动</a>
          <a>删除</a>
        </Space>
      )
    }
  ]
  const columns: TableColumnsType<DataType> = [
    { title: '存档名称', dataIndex: 'save_name' },
    { title: 'cluster_name', dataIndex: 'cluster_name' },
    // { title: 'cluster_description', dataIndex: 'cluster_description' },
    { title: '游戏模式', dataIndex: 'game_mode' },
    { title: '游戏密码', dataIndex: 'cluster_password' },
    {
      title: '操作',
      key: 'operation',
      render: () => (
        <Space>
          <a>一键启动</a>
          <a>删除</a>
        </Space>
      )
    }
  ]

  return (
    <CardSavesWrap className="card-saves-wrap">
      <Card
        className="card-saves"
        bordered={false}
        title="存档信息"
        extra={
          <Space>
            <Button>新建存档</Button>
            <Button onClick={handleGetAllSaves}>获取存档</Button>
          </Space>
        }
      >
        <Table<DataType>
          columns={columns}
          expandable={{
            expandedRowRender: record => (
              <Table<ExpandedDataType>
                columns={expandColumns}
                dataSource={record.worlds}
                pagination={false}
              />
            )
          }}
          dataSource={dataSource}
          pagination={false}
          size="small"
          bordered
        />
      </Card>
    </CardSavesWrap>
  )
}
