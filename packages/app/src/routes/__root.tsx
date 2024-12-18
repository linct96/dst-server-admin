import {
  CheckOutlined,
  CloseOutlined,
  CloudOutlined,
  DesktopOutlined,
  LoadingOutlined,
  PlusOutlined,
  UploadOutlined,
  UserOutlined,
  VideoCameraOutlined
} from '@ant-design/icons'
import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { Button, Divider, Dropdown, Layout, Menu, theme, Space } from 'antd'
import { cloneElement, ReactElement, useEffect, useState } from 'react'
import { SERVER_URL } from '../const'

function Root() {
  const { token } = theme.useToken()
  const contentStyle: React.CSSProperties = {
    backgroundColor: token.colorBgElevated,
    borderRadius: token.borderRadiusLG,
    boxShadow: token.boxShadowSecondary
  }
  useEffect(() => {
  //   const data = {
  //     host: '101.126.78.130',
  //     key: `b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
  // QyNTUxOQAAACAJToOQSukYaRukLreyGAv6K6VGaZrQ5Kb5lEP87fgzOwAAAJiKK32qiit9
  // qgAAAAtzc2gtZWQyNTUxOQAAACAJToOQSukYaRukLreyGAv6K6VGaZrQ5Kb5lEP87fgzOw
  // AAAEARlDZvdYgyN5mkU2kPqvIFUo9GQydk/o0GmbxYD+f69glOg5BK6RhpG6Qut7IYC/or
  // pUZpmtDkpvmUQ/zt+DM7AAAAFWxpbmNoYW90aW5nQE1CUC5sb2NhbA==`.replace('\n', ''),
  //     connectType: 'key'
  //   }
  //   fetch(`${SERVER_URL}/api/remote/connect`, {
  //     method: 'POST',
  //     body: JSON.stringify(data)
  //   }).then(res => {
  //     console.log(res)
  //   })
  }, [])
  return (
    <>
      <Layout style={{ height: '100%' }}>
        <Layout.Sider theme="light" collapsible>
          <div
            style={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              height: 64
            }}
          >
            <Space>
              <Dropdown.Button
                size="large"
                menu={{
                  mode: 'vertical',
                  items: [
                    {
                      label: '1',
                      key: '0',
                      children: [
                        {
                          label: '1st menu item',
                          key: '0-1',
                          icon: <CloudOutlined />
                        }
                      ]
                    },
                    {
                      label: '1st menu item',
                      key: '1',
                      icon: <CloudOutlined />
                    },
                    {
                      label: '2nd menu item',
                      key: '2',
                      icon: <DesktopOutlined />
                    }
                  ]
                }}
                dropdownRender={menu => (
                  <div style={contentStyle}>
                    {cloneElement(menu as ReactElement, {
                      style: { boxShadow: 'none' }
                    })}
                    <Divider style={{ margin: 0 }} />
                    <div style={{ padding: 8 }}>
                      <Button type="primary" style={{ width: '100%' }}>
                        添加服务器
                      </Button>
                    </div>
                  </div>
                )}
                onClick={() => {}}
              >
                <span
                  style={{
                    maxWidth: 96,
                    display: 'inline-block',
                    overflow: 'hidden',
                    textOverflow: 'ellipsis',
                    whiteSpace: 'nowrap'
                  }}
                >
                  腾讯云服务器服务器
                </span>
                {/* <LoadingOutlined /> */}
                <CloseOutlined style={{ color: token.colorError }} />
                {/* <CheckOutlined style={{ color: token.colorSuccess }} /> */}
              </Dropdown.Button>
            </Space>
          </div>
          <Menu
            mode="inline"
            defaultSelectedKeys={['1']}
            items={[
              {
                key: '1',
                icon: <UserOutlined />,
                label: 'nav 1'
              },
              {
                key: '2',
                icon: <VideoCameraOutlined />,
                label: 'nav 2'
              },
              {
                key: '3',
                icon: <UploadOutlined />,
                label: 'nav 3'
              }
            ]}
          />
        </Layout.Sider>
        <Layout>
          <Layout.Header style={{ background: '#fff' }} />
          <Layout.Content
            style={{
              padding: 24,
              minHeight: 280,
              background: ''
            }}
          >
            <Outlet />
          </Layout.Content>
        </Layout>
      </Layout>
      <TanStackRouterDevtools position="bottom-right" />
    </>
  )
}

export const Route = createRootRoute({
  component: Root
})
