import {
  Badge,
  Button,
  Card,
  Input,
  InputNumber,
  Space,
  Table,
  TableColumnsType
} from 'antd'
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
export default function CardMods() {
  const [modID, setModID] = useState('')
  const [dataSource, setDataSource] = useState<DataType[]>([])
  const init = async () => {
    await handleGetAllMods()
  }

  const handleGetAllMods = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/get_all_mods`, {
      method: 'GET'
    })
    const data = await res.json()
    console.log('handleGetAllMods', data)
    setDataSource(
      data.data.mods.map((item: any) => {
        return {
          key: item,
          id: item
        }
      })
    )
  }
  const handleAddMods = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/add_mods`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        mods: [`${modID}`]
      })
    })
    const json = await res.json()

    console.log('handleAddMods', json.data)
  }
  const handleDeleteMods = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/delete_mods`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        mods: [`${modID}`]
      })
    })
    const json = await res.json()

    console.log('handleDeleteMods', json.data)
  }
  useEffect(() => {
    init()
  }, [])

  const getExpandColumns: (
    record: DataType
  ) => TableColumnsType<ExpandedDataType> = record => {
    return [
      { title: '世界名称', dataIndex: 'id' },
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
            <a>启动</a>
            <a>停止</a>
            <a>删除</a>
          </Space>
        )
      }
    ]
  }
  const columns: TableColumnsType<DataType> = [
    { title: '模组id', dataIndex: 'id' },
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
        title="模组信息"
        extra={
          <Space>
            <InputNumber
              style={{ width: '200px' }}
              controls={false}
              value={modID}
              placeholder="mod_id"
              onChange={e => {
                if (e) {
                  setModID(e)
                }
              }}
            />
            <Button onClick={handleAddMods}>插入</Button>
            <Button
              color="danger"
              variant="outlined"
              onClick={handleDeleteMods}
            >
              删除
            </Button>
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
          rowKey={'id'}
        />
      </Card>
    </CardSavesWrap>
  )
}
