import { useState } from 'react'
import './App.css'
import { Button } from 'antd'
import { SERVER_URL } from './const'

function App() {
  const [count, setCount] = useState(0)
  const getGlobalVar = async () => {
    const res = await fetch(`${SERVER_URL}/api/getGlobalVar`)
    console.log('getGlobalVar', res)
    if (res.ok) {
      const result = await res.json()
      console.log('getGlobalVar result', result)
    }
  }
  const init = () => {
    fetch(`${SERVER_URL}/api/init`)
  }
  const createSave = async () => {
    const res = await fetch(`${SERVER_URL}/api/createSave`)
    console.log('createSave', res)
    if (res.ok) {
      const result = await res.json()
      console.log('createSave result', result)
    }
  }
  return (
    <div>
      <Button onClick={getGlobalVar}>get global var</Button>
      <Button onClick={init}>init</Button>
      <Button onClick={createSave}>create save</Button>
    </div>
  )
}

export default App
