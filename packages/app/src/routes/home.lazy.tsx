import { createLazyFileRoute } from '@tanstack/react-router'

function Home() {
  return (
    <div className="p-2">
      <h3>2home</h3>
    </div>
  )
}

export const Route = createLazyFileRoute('/home')({
  component: Home
})
