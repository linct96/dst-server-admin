import { useState } from 'react'
import './App.css'
import { Button } from 'antd'
import { SERVER_URL } from './const'

function App() {
  const [count, setCount] = useState(0)
  const installSteamCMD = () => {
    fetch(`${SERVER_URL}/api/installSteamCMD`)
  }
  return (
    <div>
      <Button onClick={installSteamCMD}>install steam_cmd</Button>
    </div>
  )
}

export default App
