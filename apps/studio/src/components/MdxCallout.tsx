import type { ReactNode } from 'react'

export function MdxCallout({
  title,
  children,
}: {
  title: string
  children: ReactNode
}) {
  return (
    <aside className="not-prose my-6 rounded-lg border bg-muted/40 p-4">
      <p className="mb-2 text-sm font-medium">{title}</p>
      <div className="text-sm leading-7 text-muted-foreground">{children}</div>
    </aside>
  )
}
