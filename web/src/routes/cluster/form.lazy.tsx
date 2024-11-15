import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/cluster/form')({
  component: ClusterForm
})

function ClusterForm() {
  return <div>321</div>
}
