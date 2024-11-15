import { createFileRoute } from '@tanstack/react-router'
import { Button, Form, FormProps, Input, InputNumber, Radio } from 'antd'
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

  useEffect(()=>{
    // fetch(`${SERVER_URL}/api/remote/connect`, {
    //   method: 'POST',
    //   body: JSON.stringify({})
    // }).then(res => {
    //   console.log(res)
    // })
  },[])
  return (
    <div className="p-2">
      12
    </div>
  )
}

export const Route = createFileRoute('/home')({
  component: Home
})
