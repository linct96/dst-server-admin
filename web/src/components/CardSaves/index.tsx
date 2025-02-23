import { Badge, Button, Card, Space, Table, TableColumnsType } from 'antd'
import { SERVER_URL } from '../../const'
import { CardSavesWrap } from './index.style'
import { useEffect, useState } from 'react'
type ExpandedDataType = {
  key: React.Key
  date: string
  name: string
  upgradeNum: string
  world: string
}
type DataType = {
  key: React.Key
  cluster: string
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
  const handleCreateSaves = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/edit_save`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        save_name: 'test'
      })
    })
    const data = await res.json()
    console.log('handleCreateSaves', data)
  }

  const handleGetAllSaves = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/get_all_saves`, {
      method: 'GET'
    })
    const data = await res.json()
    console.log('handleGetAllSaves', data)
    setDataSource(data.data)
  }
  const handleStartDstServer = async (data: {
    cluster: string
    world: string
  }) => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/start_dst_server`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    })
    const json = await res.json()

    console.log('handleStartDstServer', json.data)
  }
  const handleStopDstServer = async (data: {
    cluster: string
    world: string
  }) => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/stop_dst_server`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    })
    const json = await res.json()
    console.log(json.data)
  }
  useEffect(() => {
    init()
  }, [])

  const getExpandColumns: (
    record: DataType
  ) => TableColumnsType<ExpandedDataType> = record => {
    return [
      { title: '世界名称', dataIndex: 'world' },
      { title: 'Name', dataIndex: 'name' },
      {
        title: 'Status',
        key: 'state',
        render: () => <Badge status="success" text="Finished" />
      },
      {
        title: '操作',
        render: (_, childRecord) => (
          <Space>
            <a
              onClick={() =>
                handleStartDstServer({
                  cluster: record.cluster,
                  world: childRecord.world
                })
              }
            >
              启动
            </a>
            <a
              onClick={() =>
                handleStopDstServer({
                  cluster: record.cluster,
                  world: childRecord.world
                })
              }
            >
              停止
            </a>
            <a>删除</a>
          </Space>
        )
      }
    ]
  }
  const columns: TableColumnsType<DataType> = [
    { title: '存档名称', dataIndex: 'cluster' },
    { title: '房间名称', dataIndex: 'cluster_name' },
    { title: '房间描述', dataIndex: 'cluster_description' },
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
            <Button onClick={handleCreateSaves}>新建存档</Button>
            <Button onClick={handleGetAllSaves}>获取存档</Button>
          </Space>
        }
      >
        <Table<DataType>
          columns={columns}
          expandable={{
            expandedRowRender: record => (
              <Table<ExpandedDataType>
                columns={getExpandColumns(record)}
                dataSource={record.worlds}
                pagination={false}
                rowKey={'world'}
                bordered
              />
            )
          }}
          bordered={false}
          dataSource={dataSource}
          pagination={false}
          size="small"
          rowKey={'cluster'}
        />
      </Card>
    </CardSavesWrap>
  )
}
