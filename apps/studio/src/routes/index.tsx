import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Separator } from '@/components/ui/separator'

export const Route = createFileRoute('/')({ component: App })

function App() {
  return (
    <main className="mx-auto w-full max-w-6xl px-4 pb-8 pt-14">
      <section className="rounded-xl border bg-card px-6 py-10 sm:px-10 sm:py-14">
        <p className="mb-3 text-sm font-medium text-muted-foreground">Fastforge Studio</p>
        <h1 className="mb-5 max-w-3xl text-4xl font-semibold tracking-tight sm:text-6xl">
          Design DMG layouts visually for Fastforge.
        </h1>
        <p className="mb-8 max-w-2xl text-base text-muted-foreground sm:text-lg">
          Build and refine `make_config.yaml` with a dedicated canvas, full
          field coverage, and instant YAML export for your macOS DMG packages.
        </p>
        <div className="flex flex-wrap gap-3">
          <Button asChild className="px-5">
            <a href="/dmg-designer" className="no-underline">
              Open DMG Designer
            </a>
          </Button>
          <Button asChild variant="secondary" className="px-5">
            <a href="/blog" className="no-underline">
              Read Product Notes
            </a>
          </Button>
        </div>
      </section>

      <section className="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {[
          [
            'Direct Manipulation',
            'Drag app and link targets on a Finder-like canvas with snap and guide support.',
          ],
          [
            'Full Field Coverage',
            'Edit title, background, icon sizing, window geometry, signing, format, and filesystem.',
          ],
          [
            'YAML Round-Trip',
            'Switch between visual editing and valid `make_config.yaml` without hand-syncing fields.',
          ],
          [
            'Studio Workflow',
            'Use Studio as a focused packaging workbench instead of a generic starter homepage.',
          ],
        ].map(([title, desc], index) => (
          <Card
            key={title}
            className="p-0"
            style={{ animationDelay: `${index * 90 + 80}ms` }}
          >
            <CardHeader className="p-5 pb-2">
              <CardTitle className="text-base">{title}</CardTitle>
            </CardHeader>
            <CardContent className="p-5 pt-0">
              <p className="m-0 text-sm text-muted-foreground">{desc}</p>
            </CardContent>
          </Card>
        ))}
      </section>

      <section className="mt-8 rounded-xl border bg-card p-6">
        <p className="mb-2 text-sm font-medium text-muted-foreground">How It Works</p>
        <Separator className="mb-4" />
        <ul className="m-0 list-disc space-y-2 pl-5 text-sm text-muted-foreground">
          <li>
            Open <code>/dmg-designer</code> to edit a complete DMG spec with visual
            controls and YAML output.
          </li>
          <li>
            Drag contents on the canvas, then fine-tune positions and advanced
            options from the side panels.
          </li>
          <li>
            Paste or copy YAML whenever you need to move between Studio and your
            Fastforge project.
          </li>
        </ul>
      </section>
    </main>
  )
}
