export default function Footer() {
  const year = new Date().getFullYear()

  return (
    <footer className="mt-20 border-t px-4 py-10 text-sm text-muted-foreground">
      <div className="mx-auto flex w-full max-w-6xl flex-col items-center justify-between gap-3 text-center sm:flex-row sm:text-left">
        <p className="m-0">&copy; {year} Fastforge Studio. All rights reserved.</p>
        <p className="m-0">Packaging Workbench for macOS DMG</p>
      </div>
    </footer>
  )
}
