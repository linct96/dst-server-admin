import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/cluster/cool')({
  component: () => <div>Hello /cluster/cool!</div>
})
