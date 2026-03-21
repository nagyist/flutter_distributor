import { Link } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import ThemeToggle from './ThemeToggle'

export default function Header() {
  return (
    <header className="sticky top-0 z-50 border-b bg-background/95 px-4 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <nav className="mx-auto flex w-full max-w-6xl flex-wrap items-center gap-x-3 gap-y-2 py-3 sm:py-4">
        <h2 className="m-0 flex-shrink-0 text-base font-semibold tracking-tight">
          <Button asChild variant="ghost" size="sm" className="px-2 text-base">
            <Link to="/">
              <span className="inline-block size-2 rounded-full bg-foreground" />
              Fastforge Studio
            </Link>
          </Button>
        </h2>

        <div className="ml-auto flex items-center gap-1.5 sm:ml-0 sm:gap-2">
          <ThemeToggle />
        </div>

        <div className="order-3 flex w-full flex-wrap items-center gap-1 pb-1 text-sm sm:order-2 sm:w-auto sm:flex-nowrap sm:pb-0">
          <Link
            to="/"
            className="inline-flex items-center rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
            activeProps={{
              className:
                'inline-flex items-center rounded-md bg-accent px-3 py-2 text-accent-foreground',
            }}
          >
            Home
          </Link>
          <Link
            to="/blog"
            className="inline-flex items-center rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
            activeProps={{
              className:
                'inline-flex items-center rounded-md bg-accent px-3 py-2 text-accent-foreground',
            }}
          >
            Blog
          </Link>
          <Link
            to="/about"
            className="inline-flex items-center rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
            activeProps={{
              className:
                'inline-flex items-center rounded-md bg-accent px-3 py-2 text-accent-foreground',
            }}
          >
            About
          </Link>
          <Link
            to="/dmg-designer"
            className="inline-flex items-center rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
            activeProps={{
              className:
                'inline-flex items-center rounded-md bg-accent px-3 py-2 text-accent-foreground',
            }}
          >
            DMG Designer
          </Link>
        </div>
      </nav>
    </header>
  )
}
