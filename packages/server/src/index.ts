import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import { createServer } from 'node:http2'
import os from 'node:os'
import axios from 'axios'
import { createWriteStream } from 'fs'
import { $, execa, execaCommand } from 'execa'
import { APP_PORT, WORKING_PROCESS_KEY, WORKING_PROCESS_MAP } from './const'
import terminate from 'terminate/promise'
import { sign } from 'crypto'
import process from 'node:process'
import { streamText, streamSSE, stream } from 'hono/streaming'
import app from './app'
// const app = new Hono()
// app.get('/', async c => {
//   return c.text('hello world')
// })
// app.notFound(async c => {
//   return c.text('Not found', 404)
// })
console.log('platform:', os.platform())
serve({
  fetch: app.fetch,
  port: APP_PORT
})
console.log(`Server is running on port ${APP_PORT}`)
// const controller = new AbortController()
// const app = new Hono()

// const startServer = async () => {
//   const childProcess = $({
//     shell: true
//   })`ping cip.cc`

//   const pid = childProcess.pid
//   if (pid) {
//     WORKING_PROCESS_MAP.set(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD, childProcess)
//     try {
//       for await (const line of childProcess) {
//         console.warn(line)
//       }
//     } catch (error) {
//       // console.log('eeeee', error)
//     }
//   }
//   WORKING_PROCESS_MAP.delete(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)

//   return true
// }

// app.get('/stop', async c => {
//   const childProcess = WORKING_PROCESS_MAP.get(
//     WORKING_PROCESS_KEY.INSTALL_STEAM_CMD
//   )
//   if (childProcess && childProcess.pid) {
//     try {
//       await terminate(childProcess.pid)
//       WORKING_PROCESS_MAP.delete(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)
//       return c.text('Process terminated successfully')
//     } catch (error) {
//       console.error('eeee', error)
//     }
//   } else {
//     return c.text('No process is running')
//   }
// })
// app.get('/start', async c => {
//   // const writer = createWriteStream('./a.tar.gz');
//   if (WORKING_PROCESS_MAP.get(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)) {
//     return c.text('Progress is already in progress')
//   } else {
//     c.header('Content-Type', 'text/event-stream')
//     return stream(c, async stream => {
//       // Write a text with a new line ('\n').
//       const childProcess = $({
//         shell: true
//       })`ping cip.cc`
//       const pid = childProcess.pid
//       if (pid) {
//         WORKING_PROCESS_MAP.set(
//           WORKING_PROCESS_KEY.INSTALL_STEAM_CMD,
//           childProcess
//         )
//         try {
//           for await (const line of childProcess) {
//             console.warn(line)
//             await stream.writeln(line)
//             // await stream.writeln(line)
//           }
//         } catch (error) {
//           // console.log('eeeee', error)
//         }
//       }
//       WORKING_PROCESS_MAP.delete(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)
//       // await stream.writeln('Hello')
//       // // Wait 1 second.
//       // await stream.sleep(1000)
//       // // Write a text without a new line.
//       // await stream.write(`Hono!`)
//     })
//     // startServer()
//     const childProcess = $({
//       shell: true
//     })`ping cip.cc`

//     const pid = childProcess.pid
//     if (pid) {
//       WORKING_PROCESS_MAP.set(
//         WORKING_PROCESS_KEY.INSTALL_STEAM_CMD,
//         childProcess
//       )
//       try {
//         for await (const line of childProcess) {
//           console.warn(line)
//         }
//       } catch (error) {
//         // console.log('eeeee', error)
//       }
//     }
//     WORKING_PROCESS_MAP.delete(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)
//   }

//   return c.text('Hello Hono!')
// })
// app.get('/', async c => {
//   // const writer = createWriteStream('./a.tar.gz');
//   // if (WORKING_PROCESS_MAP.get(WORKING_PROCESS_KEY.INSTALL_STEAM_CMD)) {
//   //   return c.text('Progress is already in progress')
//   // } else {
//   //   startServer()
//   // }

//   // axios.get("https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz",{
//   //   responseType:"stream"
//   // }).then(res=>{
//   //   res.data.pipe(writer)
//   // })
//   // for await (const line of $({ shell: true })`echo $\{HOME\}`) {
//   //   console.warn(line)
//   // }

//   // try {
//   //   if (progress) {
//   //     return c.text('Progress is already in progress')
//   //   }
//   //   progress = true
//   //   for await (const line of $({ shell: true })`ping cip.cc`) {
//   //     console.warn(line)
//   //   }
//   // } catch (e) {
//   //   console.error(e)
//   // }

//   // execa('echo ${HOME}', { shell: true }).then(result => {
//   //   console.log(result.stdout)
//   // })

//   // execa('ping cip.cc', { shell: true }).then(result => {
//   //   console.log(result.stdout)
//   // })
//   // await $`curl -qL https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz`
//   return c.html(`<!doctype html>
//   <html>
//     <head>
//       <script src="https://cdnjs.cloudflare.com/ajax/libs/axios/1.7.2/axios.min.js" integrity="sha512-JSCFHhKDilTRRXe9ak/FJ28dcpOJxzQaCd3Xg8MyF6XFjODhy/YMCM8HW0TFDckNHWUewW+kfvhin43hKtJxAw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
//       <script src="https://cdn.jsdelivr.net/npm/xterm@5.3.0/lib/xterm.min.js"></script>
//       <link href="https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.min.css" rel="stylesheet">
//     </head>
//     <body>
//       <div id="terminal"></div>
//       <script>
//         var term = new Terminal();
//         term.open(document.getElementById('terminal'));
//         // term.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m')
//         // term.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m')
//         (async ()=>{
//           const response = await fetch('http://101.126.78.130:3000/start')
//           console.log(response)
//           console.log(response.body)
//           const reader = response.body.getReader();
//           while (true) {
//             const { done, value } = await reader.read();

//             if (done) {
//               console.log("Stream complete");
//               break;
//             }
//             const text = new TextDecoder().decode(value)
//             term.writeln(text)
//             console.log(text)
//           }
//         })();

//         // axios.get('http://101.126.78.130:3000/start', {
//         //   headers: {
//         //     'accept': '*',
//         //     'content-type': 'application/json'
//         //   },
//         //   onDownloadProgress: progressEvent => {
//         //     const xhr = progressEvent.event.target
//         //     const { responseText } = xhr
//         //     console.log("=====responseText======")
//         //     console.log(responseText)
//         //   }
//         // })

//       </script>
//     </body>
//   </html>`)
// })
