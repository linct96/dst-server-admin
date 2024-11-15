import React from 'react'
import { createRoot } from 'react-dom/client'
import RouterRegister from './RouterRegister'
import './index.css'

const $root = document.getElementById('root')
if ($root) {
  createRoot($root).render(
    <React.StrictMode>
      <RouterRegister />
    </React.StrictMode>
  )
}
