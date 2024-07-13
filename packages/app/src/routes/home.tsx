import { createFileRoute } from '@tanstack/react-router'
import { Button, Form, FormProps, Input, InputNumber, Radio } from 'antd'
import { SERVER_URL } from '../const'

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
  const [form] = Form.useForm<FieldType>()
  const connectType = Form.useWatch(['connectType'], form)
  const onFinish: FormProps<FieldType>['onFinish'] = values => {
    const resolvedValues = {
      ...values,
      port: values.port || 22
    }
    fetch(`${SERVER_URL}/api/remote/connect`, {
      method: 'POST',
      body: JSON.stringify(resolvedValues)
    }).then(res => {
      console.log(res)
    })
    console.log('resolvedValues', resolvedValues)
  }

  const onFinishFailed: FormProps<FieldType>['onFinishFailed'] = errorInfo => {
    console.log('Failed:', errorInfo)
  }
  return (
    <div className="p-2">
      <div className="login-form" style={{ width: '400px' }}>
        <Form
          form={form}
          name="basic"
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          style={{ maxWidth: 600 }}
          initialValues={initialValues}
          onFinish={onFinish}
          onFinishFailed={onFinishFailed}
          autoComplete="off"
        >
          <Form.Item name="host" label="主机ip">
            <Input placeholder="请输入主机ip" />
          </Form.Item>
          <Form.Item name="port" label="端口">
            <InputNumber placeholder="默认为22" controls={false} />
          </Form.Item>
          <Form.Item label="登录方式">
            {/* <Input placeholder="请输入密码" /> */}
            <Form.Item name="connectType" noStyle>
              <Radio.Group buttonStyle="solid">
                <Radio.Button value="password">密码</Radio.Button>
                <Radio.Button value="key">密钥</Radio.Button>
              </Radio.Group>
            </Form.Item>
            {connectType === 'key' && (
              <Form.Item name="key" style={{ marginTop: '12px' }} noStyle>
                <Input.TextArea placeholder="请输入密钥" />
              </Form.Item>
            )}
            {connectType === 'password' && (
              <Form.Item name="password" style={{ marginTop: '12px' }} noStyle>
                <Input placeholder="请输入密码" />
              </Form.Item>
            )}
          </Form.Item>

          <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
            <Button type="primary" htmlType="submit">
              连接
            </Button>
          </Form.Item>
        </Form>
      </div>
    </div>
  )
}

export const Route = createFileRoute('/home')({
  component: Home
})
