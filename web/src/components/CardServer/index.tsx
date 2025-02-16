import { Button, Card, Descriptions, Space } from "antd";
import { SERVER_URL } from "../../const";
import { useRequest } from "ahooks";

export default function CardServer() {
  const { data, loading, refresh } = useRequest(
    async () => {
      const response = await fetch(
        `${SERVER_URL}/api/auth/game/get_game_info`
      )
      const json = await response.json()
      return json.data
    },
    {
      // pollingInterval: 1000 * 2
    }
  )
  const handleForceInstall = async () => {
    const res = await fetch(`${SERVER_URL}/api/auth/game/install_dedicated_server`, {
      method: 'POST',
      body: JSON.stringify({ force: false }),
      headers: {
        'Content-Type': 'application/json'
      }
    })
    const data = await res.json()
    console.log('handleForceInstall', data)
  }
  const handleUpdateGame = async () => {
    const res = await fetch(
      `${SERVER_URL}/api/auth/game/update_dedicated_server`,
      {
        method: 'POST',
        body: JSON.stringify({})
      }
    )
    const data = await res.json()
    console.log('handleUpdateGame', data)
  }

  console.log('CardServer',data)
  if (!data) return null
  return <Card
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
              <span>{data.version}</span>
              <a onClick={handleUpdateGame}>更新游戏</a>
            </Space>
          )
        }
      ]}
      style={{ width: '320px' }}
    />
  </Card>
}