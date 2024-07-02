import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import axios from 'axios'
import { createWriteStream } from 'fs'
import { $ } from 'execa';

const app = new Hono()

app.get('/', async (c) => {
  // const writer = createWriteStream('./a.tar.gz');

  // axios.get("https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz",{
  //   responseType:"stream"
  // }).then(res=>{
  //   res.data.pipe(writer)
  // })

  // await $`curl -qL https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz`


  return c.text('Hello Hono!')
})

const port = 3000
console.log(`Server is running on port ${port}`)

serve({
  fetch: app.fetch,
  port
})
