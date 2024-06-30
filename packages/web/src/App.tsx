import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }))
  }

  useEffect(() => {
    const rtc = new RTCPeerConnection()
    console.log('rtc', rtc)
  }, [])

  return (
    <div className="container">
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={e => {
          e.preventDefault()
          greet()
        }}
      >
        <input
          id="greet-input"
          onChange={e => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
      <div style={{ display: 'flex', justifyContent: 'center' }}>
        <button
          type="submit"
          onClick={() => {
            invoke('test_function')
          }}
        >
          test
        </button>
      </div>
    </div>
  )
}

export default App
