import { type ReactNode, useEffect, useId, useState } from 'react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { Textarea } from '@/components/ui/textarea'
import { cn } from '@/lib/utils'
import {
  DMG_CONTENT_TYPES,
  DMG_FILESYSTEMS,
  DMG_FORMATS,
  type DmgContentItem,
  type DmgContentType,
  type DmgSpec,
  createContentItem,
  createDefaultDmgSpec,
  pruneEmptyValues,
  toSerializableSpec,
  validateDmgSpec,
} from './model'
import { specToYaml, yamlToSpec } from './yaml'

type GuideLine = {
  orientation: 'vertical' | 'horizontal'
  position: number
}

type DragState = {
  pointerId: number
  itemId: string
  startClientX: number
  startClientY: number
  originX: number
  originY: number
}

const CANVAS_PADDING = 24
const GRID_SIZE = 16
const GUIDE_THRESHOLD = 10

export function DmgDesigner() {
  const [spec, setSpec] = useState<DmgSpec>(() => createDefaultDmgSpec())
  const [selectedId, setSelectedId] = useState<string | null>(null)
  const [zoom, setZoom] = useState(1)
  const [snapToGrid, setSnapToGrid] = useState(true)
  const [yamlDraft, setYamlDraft] = useState(() => specToYaml(createDefaultDmgSpec()))
  const [yamlMessage, setYamlMessage] = useState<string | null>(null)
  const [guides, setGuides] = useState<GuideLine[]>([])
  const [dragState, setDragState] = useState<DragState | null>(null)
  const [copied, setCopied] = useState(false)
  const toolbarId = useId()

  useEffect(() => {
    if (!selectedId && spec.contents.length > 0) {
      setSelectedId(spec.contents[0]?.id ?? null)
    }
  }, [selectedId, spec.contents])

  useEffect(() => {
    setYamlDraft(specToYaml(spec))
  }, [spec])

  useEffect(() => {
    if (!copied) {
      return undefined
    }

    const timeout = window.setTimeout(() => setCopied(false), 1400)
    return () => window.clearTimeout(timeout)
  }, [copied])

  useEffect(() => {
    function onPointerMove(event: PointerEvent) {
      setDragState((state) => {
        if (!state) {
          return state
        }

        setSpec((current) => {
          const next = {
            ...current,
            contents: current.contents.map((item) => ({ ...item })),
          }

          const item = next.contents.find((entry) => entry.id === state.itemId)
          if (!item) {
            return current
          }

          const deltaX = (event.clientX - state.startClientX) / zoom
          const deltaY = (event.clientY - state.startClientY) / zoom
          const rawX = state.originX + deltaX
          const rawY = state.originY + deltaY
          const snapped = computeSnappedPosition(
            {
              x: rawX,
              y: rawY,
            },
            next,
            item.id,
            snapToGrid,
          )

          item.x = snapped.x
          item.y = snapped.y
          setGuides(snapped.guides)

          return next
        })

        return state
      })
    }

    function onPointerUp(event: PointerEvent) {
      setDragState((state) => {
        if (!state || state.pointerId !== event.pointerId) {
          return state
        }

        setGuides([])
        return null
      })
    }

    window.addEventListener('pointermove', onPointerMove)
    window.addEventListener('pointerup', onPointerUp)
    return () => {
      window.removeEventListener('pointermove', onPointerMove)
      window.removeEventListener('pointerup', onPointerUp)
    }
  }, [snapToGrid, zoom])

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (!selectedId) {
        return
      }

      const activeElement = document.activeElement
      const isTyping =
        activeElement instanceof HTMLInputElement ||
        activeElement instanceof HTMLTextAreaElement ||
        activeElement instanceof HTMLSelectElement
      if (isTyping) {
        return
      }

      const step = event.shiftKey ? GRID_SIZE : 1
      let deltaX = 0
      let deltaY = 0

      if (event.key === 'ArrowLeft') {
        deltaX = -step
      } else if (event.key === 'ArrowRight') {
        deltaX = step
      } else if (event.key === 'ArrowUp') {
        deltaY = -step
      } else if (event.key === 'ArrowDown') {
        deltaY = step
      } else {
        return
      }

      event.preventDefault()
      updateSelectedItem((item) => ({
        ...item,
        x: clamp(Math.round(item.x + deltaX), 0, getCanvasWidth(spec)),
        y: clamp(Math.round(item.y + deltaY), 0, getCanvasHeight(spec)),
      }))
    }

    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [selectedId, spec])

  const validation = validateDmgSpec(spec)
  const selectedItem =
    spec.contents.find((item) => item.id === selectedId) ?? spec.contents[0] ?? null
  const canvasWidth = getCanvasWidth(spec)
  const canvasHeight = getCanvasHeight(spec)
  const panelClass =
    'min-h-0 min-w-0 rounded-xl border bg-card p-5 shadow-sm xl:h-full xl:overflow-auto'

  function updateSpec(updater: (current: DmgSpec) => DmgSpec) {
    setSpec((current) => updater(current))
  }

  function updateSelectedItem(
    updater: (item: DmgContentItem) => DmgContentItem,
    explicitId?: string,
  ) {
    const targetId = explicitId ?? selectedId
    if (!targetId) {
      return
    }

    updateSpec((current) => ({
      ...current,
      contents: current.contents.map((item) =>
        item.id === targetId ? updater(item) : item,
      ),
    }))
  }

  function addContentItem(type: DmgContentType) {
    const item = createContentItem({
      type,
      path: type === 'link' ? '/Applications' : 'MyAsset.app',
      x: 240,
      y: 220,
      name: type === 'link' ? 'Applications' : undefined,
    })
    updateSpec((current) => ({
      ...current,
      contents: [...current.contents, item],
    }))
    setSelectedId(item.id)
  }

  function removeSelectedItem() {
    if (!selectedItem) {
      return
    }

    updateSpec((current) => ({
      ...current,
      contents: current.contents.filter((item) => item.id !== selectedItem.id),
    }))
    setSelectedId((current) => {
      if (current !== selectedItem.id) {
        return current
      }
      const remaining = spec.contents.filter((item) => item.id !== selectedItem.id)
      return remaining[0]?.id ?? null
    })
  }

  function moveSelectedItem(direction: -1 | 1) {
    if (!selectedItem) {
      return
    }

    updateSpec((current) => {
      const index = current.contents.findIndex((item) => item.id === selectedItem.id)
      if (index < 0) {
        return current
      }
      const nextIndex = index + direction
      if (nextIndex < 0 || nextIndex >= current.contents.length) {
        return current
      }

      const items = current.contents.map((item) => ({ ...item }))
      const [moved] = items.splice(index, 1)
      items.splice(nextIndex, 0, moved)
      return {
        ...current,
        contents: items,
      }
    })
  }

  async function copyYaml() {
    await navigator.clipboard.writeText(yamlDraft)
    setCopied(true)
  }

  function applyYaml() {
    const result = yamlToSpec(yamlDraft)
    if (!result.ok) {
      setYamlMessage(result.message)
      return
    }

    setSpec(result.spec)
    setSelectedId(result.spec.contents[0]?.id ?? null)
    setYamlMessage(
      result.errors.length > 0
        ? `${result.errors.length} validation issue(s) found in imported YAML.`
        : 'YAML imported successfully.',
    )
  }

  function resetToDefault() {
    const next = createDefaultDmgSpec()
    setSpec(next)
    setSelectedId(next.contents[0]?.id ?? null)
    setYamlMessage('Reset to default DMG layout.')
  }

  return (
    <main className="min-h-screen p-0 xl:h-screen">
      <section className="grid h-full grid-rows-[auto_minmax(0,1fr)] gap-4 p-4 xl:h-full">
        <header className="flex min-h-[100px] flex-col justify-between gap-4 rounded-xl border bg-card px-5 py-5 shadow-sm lg:flex-row lg:items-center">
          <div className="min-w-0">
            <p className="mb-2 text-sm font-medium text-muted-foreground">Fastforge Studio</p>
            <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl">
              DMG Designer
            </h1>
            <p className="mt-3 max-w-2xl text-sm text-muted-foreground">
              A full-screen DMG configuration workbench with canvas editing,
              precision controls, and YAML round-tripping.
            </p>
          </div>
          <div
            className="flex flex-wrap items-center gap-3 rounded-lg border bg-background p-3"
            aria-labelledby={toolbarId}
          >
            <span id={toolbarId} className="sr-only">
              Designer controls
            </span>
            <label className="inline-flex items-center gap-2 text-[0.82rem] text-muted-foreground">
              <span className="font-medium">Zoom</span>
              <input
                type="range"
                min="0.6"
                max="1.6"
                step="0.05"
                value={zoom}
                onChange={(event) => setZoom(Number(event.target.value))}
                className="w-[110px] accent-foreground"
              />
              <strong>{Math.round(zoom * 100)}%</strong>
            </label>
            <label className="inline-flex items-center gap-2 text-[0.82rem] text-muted-foreground">
              <input
                type="checkbox"
                checked={snapToGrid}
                onChange={(event) => setSnapToGrid(event.target.checked)}
                className="size-4 accent-foreground"
              />
              <span>Snap to grid</span>
            </label>
            <Button type="button" variant="secondary" size="sm" onClick={resetToDefault}>
              Reset layout
            </Button>
          </div>
        </header>

        <section className="grid h-full min-h-0 grid-cols-1 gap-4 xl:grid-cols-[minmax(0,320px)_minmax(0,1fr)_minmax(0,340px)]">
          <aside className={panelClass}>
            <div className="grid gap-4 sm:grid-cols-1 sm:items-start sm:justify-between xl:flex">
              <div>
                <p className="mb-2 text-sm font-medium text-muted-foreground">Global</p>
                <h2 className="text-lg font-semibold">
                DMG Configuration
                </h2>
              </div>
            <div>
              <Badge variant={validation.isValid ? 'default' : 'destructive'}>
                {validation.isValid ? 'Valid' : `${validation.errors.length} issue(s)`}
              </Badge>
            </div>
          </div>

          <div className="mt-5 grid gap-4">
            <TextField
              label="Title"
              value={spec.title}
              onChange={(value) => updateSpec((current) => ({ ...current, title: value }))}
            />
            <TextField
              label="Icon Path"
              value={spec.icon ?? ''}
              placeholder="assets/app.icns"
              onChange={(value) =>
                updateSpec((current) => ({
                  ...current,
                  icon: value || undefined,
                }))
              }
            />
            <TextField
              label="Background Image"
              value={spec.background ?? ''}
              placeholder="assets/background.png"
              onChange={(value) =>
                updateSpec((current) => ({
                  ...current,
                  background: value || undefined,
                }))
              }
            />
            <TextField
              label="Background Color"
              value={spec.backgroundColor ?? ''}
              placeholder="#0f172a"
              onChange={(value) =>
                updateSpec((current) => ({
                  ...current,
                  backgroundColor: value || undefined,
                }))
              }
            />
            <NumberField
              label="Icon Size"
              value={spec.iconSize}
              min={16}
              max={512}
              onChange={(value) =>
                updateSpec((current) => ({
                  ...current,
                  iconSize: value,
                }))
              }
            />
            <SelectField
              label="Format"
              value={spec.format ?? ''}
              options={DMG_FORMATS}
              onChange={(value) =>
                updateSpec((current) => ({
                  ...current,
                  format: (value || undefined) as DmgSpec['format'],
                }))
              }
            />
            <SelectField
              label="Filesystem"
              value={spec.filesystem ?? ''}
              options={DMG_FILESYSTEMS}
              onChange={(value) =>
                updateSpec((current) => ({
                  ...current,
                  filesystem: (value || undefined) as DmgSpec['filesystem'],
                }))
              }
            />
          </div>

          <ToggleSection
            title="Window"
            description="Finder window position and size."
            enabled={Boolean(spec.window)}
            onToggle={(checked) =>
              updateSpec((current) => ({
                ...current,
                window: checked
                  ? {
                      position: { x: 120, y: 120 },
                      size: { width: 640, height: 420 },
                    }
                  : undefined,
              }))
            }
          >
            <div className="grid gap-4 sm:grid-cols-2">
              <NumberField
                label="Window X"
                value={spec.window?.position?.x}
                onChange={(value) =>
                  updateSpec((current) => ({
                    ...current,
                    window: {
                      ...current.window,
                      position: value == null
                        ? undefined
                        : {
                            x: value,
                            y: current.window?.position?.y ?? 120,
                          },
                    },
                  }))
                }
              />
              <NumberField
                label="Window Y"
                value={spec.window?.position?.y}
                onChange={(value) =>
                  updateSpec((current) => ({
                    ...current,
                    window: {
                      ...current.window,
                      position: value == null
                        ? undefined
                        : {
                            x: current.window?.position?.x ?? 120,
                            y: value,
                          },
                    },
                  }))
                }
              />
              <NumberField
                label="Window Width"
                value={spec.window?.size?.width}
                min={320}
                onChange={(value) =>
                  updateSpec((current) => ({
                    ...current,
                    window: {
                      ...current.window,
                      size: value == null
                        ? undefined
                        : {
                            width: value,
                            height: current.window?.size?.height ?? 420,
                          },
                    },
                  }))
                }
              />
              <NumberField
                label="Window Height"
                value={spec.window?.size?.height}
                min={240}
                onChange={(value) =>
                  updateSpec((current) => ({
                    ...current,
                    window: {
                      ...current.window,
                      size: value == null
                        ? undefined
                        : {
                            width: current.window?.size?.width ?? 640,
                            height: value,
                          },
                    },
                  }))
                }
              />
            </div>
          </ToggleSection>

          <ToggleSection
            title="Code Signing"
            description="Optional DMG signing metadata."
            enabled={Boolean(spec.codeSign)}
            onToggle={(checked) =>
              updateSpec((current) => ({
                ...current,
                codeSign: checked
                  ? {
                      signingIdentity: '',
                    }
                  : undefined,
              }))
            }
          >
            <div className="grid gap-4">
              <TextField
                label="Signing Identity"
                value={spec.codeSign?.signingIdentity ?? ''}
                placeholder="Developer ID Application: Your Team"
                onChange={(value) =>
                  updateSpec((current) => ({
                    ...current,
                    codeSign: current.codeSign
                      ? {
                          ...current.codeSign,
                          signingIdentity: value,
                        }
                      : current.codeSign,
                  }))
                }
              />
              <TextField
                label="Identifier"
                value={spec.codeSign?.identifier ?? ''}
                placeholder="dev.fastforge.myapp"
                onChange={(value) =>
                  updateSpec((current) => ({
                    ...current,
                    codeSign: current.codeSign
                      ? {
                          ...current.codeSign,
                          identifier: value || undefined,
                        }
                      : current.codeSign,
                  }))
                }
              />
            </div>
          </ToggleSection>
        </aside>

        <section className={cn(panelClass, 'order-first xl:order-none')}>
          <div className="grid gap-3 sm:grid-cols-1 sm:items-start sm:justify-between xl:flex">
            <div>
              <p className="mb-2 text-sm font-medium text-muted-foreground">Canvas</p>
              <h2 className="text-lg font-semibold">
                Layout Preview
              </h2>
            </div>
            <p className="text-xs text-muted-foreground">
              Arrow keys nudge by 1px. Hold Shift for {GRID_SIZE}px.
            </p>
          </div>

          <div className="relative mt-5 min-h-[min(72vh,860px)] overflow-hidden rounded-xl border bg-muted/30 pt-10 pr-4 pb-4 pl-10 sm:pt-10 sm:pr-4 sm:pb-4 sm:pl-10">
            <div className="absolute top-0 left-10 right-0 z-[1] h-6 border-b bg-[linear-gradient(to_right,var(--border)_1px,transparent_1px),linear-gradient(to_bottom,var(--border)_1px,transparent_1px)] [background-size:16px_16px]" />
            <div className="absolute top-10 left-0 bottom-0 z-[1] w-6 border-r bg-[linear-gradient(to_right,var(--border)_1px,transparent_1px),linear-gradient(to_bottom,var(--border)_1px,transparent_1px)] [background-size:16px_16px]" />
            <div className="max-w-full overflow-auto">
              <div
                className="relative"
                style={{
                  width: canvasWidth * zoom,
                  height: canvasHeight * zoom,
                }}
              >
                <div
                  className="relative overflow-hidden rounded-xl border bg-background shadow-sm"
                  style={{
                    width: canvasWidth,
                    height: canvasHeight,
                    transform: `scale(${zoom})`,
                    transformOrigin: 'top left',
                    background: buildCanvasBackground(spec),
                  }}
                >
                  <div className="absolute inset-0 bg-[linear-gradient(var(--border)_1px,transparent_1px),linear-gradient(90deg,var(--border)_1px,transparent_1px)] [background-size:16px_16px] opacity-40" />
                  {guides.map((guide, index) => (
                    <div
                      key={`${guide.orientation}-${guide.position}-${index}`}
                      className={cn(
                        'pointer-events-none absolute z-[3] bg-primary',
                        guide.orientation === 'vertical'
                          ? 'top-0 bottom-0 w-px'
                          : 'left-0 right-0 h-px',
                      )}
                      style={
                        guide.orientation === 'vertical'
                          ? { left: guide.position }
                          : { top: guide.position }
                      }
                    />
                  ))}
                  <div className="absolute inset-0 shadow-[inset_0_0_0_1px_var(--border)]" />

                  {spec.contents.map((item) => {
                    const isSelected = item.id === selectedItem?.id
                    return (
                      <button
                        key={item.id}
                        type="button"
                        className="absolute z-[4] grid -translate-x-1/2 -translate-y-1/2 justify-items-center gap-[0.55rem] bg-transparent"
                        style={{
                          left: item.x,
                          top: item.y,
                          width: spec.iconSize ?? 128,
                        }}
                        onClick={() => setSelectedId(item.id)}
                        onPointerDown={(event) => {
                          event.preventDefault()
                          setSelectedId(item.id)
                          setDragState({
                            pointerId: event.pointerId,
                            itemId: item.id,
                            startClientX: event.clientX,
                            startClientY: event.clientY,
                            originX: item.x,
                            originY: item.y,
                          })
                        }}
                      >
                        <span
                          className={cn(
                            'grid size-[74px] place-items-center rounded-xl border bg-background text-foreground shadow-sm sm:size-16',
                            item.type === 'file' && 'bg-card',
                            item.type === 'link' && 'bg-secondary',
                            item.type === 'position' && 'bg-muted',
                            isSelected &&
                              'outline-[3px] outline-primary/40 outline-offset-4',
                          )}
                        >
                          {getTypeGlyph(item.type)}
                        </span>
                        <span className="max-w-[140px] rounded-md border bg-background px-[0.55rem] py-[0.22rem] text-center text-[0.74rem] font-medium shadow-sm">
                          {item.name?.trim() || inferLabel(item)}
                        </span>
                      </button>
                    )
                  })}
                </div>
              </div>
            </div>
          </div>

          <div className="mt-5 border-t border-border/60 pt-5">
            <div className="grid gap-3 sm:grid-cols-1 sm:items-center sm:justify-between xl:flex">
              <h3 className="text-sm font-semibold">Contents</h3>
              <div className="flex flex-wrap gap-2">
                <Button type="button" variant="secondary" size="sm" onClick={() => addContentItem('file')}>
                  Add File
                </Button>
                <Button type="button" variant="secondary" size="sm" onClick={() => addContentItem('link')}>
                  Add Link
                </Button>
                <Button
                  type="button"
                  variant="secondary"
                  size="sm"
                  onClick={() => addContentItem('position')}
                >
                  Add Position
                </Button>
              </div>
            </div>

            <div className="mt-4 grid gap-3">
              {spec.contents.map((item, index) => (
                <button
                  key={item.id}
                  type="button"
                  className={cn(
                    'flex w-full items-center justify-between gap-4 rounded-lg border bg-background px-4 py-3 text-left transition hover:bg-accent/50',
                    item.id === selectedItem?.id &&
                      'border-primary/40 bg-accent/30',
                  )}
                  onClick={() => setSelectedId(item.id)}
                >
                  <span className="grid gap-1">
                    <strong>{item.name?.trim() || inferLabel(item)}</strong>
                    <small className="text-[0.77rem] text-muted-foreground">{item.type}</small>
                  </span>
                  <span className="text-[0.77rem] text-muted-foreground">
                    {Math.round(item.x)}, {Math.round(item.y)} • #{index + 1}
                  </span>
                </button>
              ))}
            </div>
          </div>

          {selectedItem ? (
            <div className="mt-5 border-t border-border/60 pt-5">
              <div className="grid gap-3 sm:grid-cols-1 sm:items-center sm:justify-between xl:flex">
                <h3 className="text-sm font-semibold">
                  Selected Item
                </h3>
                <div className="flex flex-wrap gap-2">
                  <Button type="button" variant="secondary" size="sm" onClick={() => moveSelectedItem(-1)}>
                    Move Up
                  </Button>
                  <Button type="button" variant="secondary" size="sm" onClick={() => moveSelectedItem(1)}>
                    Move Down
                  </Button>
                  <Button type="button" variant="destructive" size="sm" onClick={removeSelectedItem}>
                    Delete
                  </Button>
                </div>
              </div>
              <div className="mt-4 grid gap-4">
                <SelectField
                  label="Type"
                  value={selectedItem.type}
                  options={DMG_CONTENT_TYPES}
                  onChange={(value) =>
                    updateSelectedItem((item) => ({
                      ...item,
                      type: (value || 'file') as DmgContentType,
                    }))
                  }
                />
                <TextField
                  label="Path"
                  value={selectedItem.path}
                  onChange={(value) =>
                    updateSelectedItem((item) => ({
                      ...item,
                      path: value,
                    }))
                  }
                />
                <TextField
                  label="Display Name"
                  value={selectedItem.name ?? ''}
                  placeholder="Optional label override"
                  onChange={(value) =>
                    updateSelectedItem((item) => ({
                      ...item,
                      name: value || undefined,
                    }))
                  }
                />
              </div>
              <div className="mt-4 grid gap-4 sm:grid-cols-2">
                <NumberField
                  label="X"
                  value={selectedItem.x}
                  onChange={(value) =>
                    updateSelectedItem((item) => ({
                      ...item,
                      x: clamp(value ?? item.x, 0, canvasWidth),
                    }))
                  }
                />
                <NumberField
                  label="Y"
                  value={selectedItem.y}
                  onChange={(value) =>
                    updateSelectedItem((item) => ({
                      ...item,
                      y: clamp(value ?? item.y, 0, canvasHeight),
                    }))
                  }
                />
              </div>
            </div>
          ) : null}
        </section>

        <aside className={panelClass}>
          <div className="grid gap-3 sm:grid-cols-1 sm:items-center sm:justify-between xl:flex">
            <div>
              <p className="mb-2 text-sm font-medium text-muted-foreground">YAML</p>
              <h2 className="text-lg font-semibold">
                make_config.yaml
              </h2>
            </div>
            <div className="flex items-center gap-2">
              <Button type="button" variant="secondary" size="sm" onClick={copyYaml}>
                {copied ? 'Copied' : 'Copy'}
              </Button>
              <Button type="button" variant="secondary" size="sm" onClick={applyYaml}>
                Apply YAML
              </Button>
            </div>
          </div>

          <Textarea
            className="mt-5 min-h-[360px] resize-y font-mono text-[0.8rem] leading-[1.55]"
            value={yamlDraft}
            onChange={(event) => {
              setYamlDraft(event.target.value)
              setYamlMessage(null)
            }}
            spellCheck={false}
          />

          <div className="mt-4">
            {yamlMessage ? (
              <p className="text-[0.77rem] text-muted-foreground">{yamlMessage}</p>
            ) : (
              <p className="text-[0.77rem] text-muted-foreground">
                YAML updates live from the visual editor. Use Apply YAML to
                import manual changes back into the canvas.
              </p>
            )}
          </div>

          <div className="mt-4 border-t border-border/60 pt-4">
            <h3 className="text-sm font-semibold">
              Validation
            </h3>
            {validation.errors.length === 0 ? (
              <p className="mt-2 text-[0.77rem] text-muted-foreground">
                No validation issues.
              </p>
            ) : (
              <ul className="mt-3 pl-4 text-[0.8rem] text-[#8d2d2d]">
                {validation.errors.map((error) => (
                  <li key={`${error.field}-${error.message}`}>
                    <code>{error.field}</code> {error.message}
                  </li>
                ))}
              </ul>
            )}
          </div>

          <div className="mt-5 border-t border-border/60 pt-5">
            <h3 className="text-sm font-semibold">
              Effective Payload
            </h3>
            <Separator className="mt-3 mb-4" />
            <pre className="overflow-auto rounded-[18px] border border-border bg-[#142032] p-4 text-[0.75rem] text-[#e4edff]">
              {JSON.stringify(pruneEmptyValues(toSerializableSpec(spec)), null, 2)}
            </pre>
          </div>
        </aside>
        </section>
      </section>
    </main>
  )
}

function ToggleSection({
  title,
  description,
  enabled,
  onToggle,
  children,
}: {
  title: string
  description: string
  enabled: boolean
  onToggle: (enabled: boolean) => void
  children: ReactNode
}) {
  return (
    <section className="mt-6 border-t border-border/60 pt-5">
      <div className="grid gap-3 sm:grid-cols-1 sm:items-start sm:justify-between xl:flex">
        <div>
          <h3 className="text-sm font-semibold">{title}</h3>
          <p className="mt-1 text-xs text-muted-foreground">{description}</p>
        </div>
        <label className="inline-flex items-center gap-2 text-[0.82rem] text-muted-foreground">
          <input
            type="checkbox"
            checked={enabled}
            onChange={(event) => onToggle(event.target.checked)}
            className="size-4 accent-foreground"
          />
          <span>{enabled ? 'Enabled' : 'Disabled'}</span>
        </label>
      </div>
      {enabled ? <div className="mt-4">{children}</div> : null}
    </section>
  )
}

function TextField({
  label,
  value,
  onChange,
  placeholder,
}: {
  label: string
  value: string
  onChange: (value: string) => void
  placeholder?: string
}) {
  return (
    <label className="grid gap-1.5">
      <Label>{label}</Label>
      <Input
        type="text"
        value={value}
        placeholder={placeholder}
        onChange={(event) => onChange(event.target.value)}
      />
    </label>
  )
}

function NumberField({
  label,
  value,
  onChange,
  min,
  max,
}: {
  label: string
  value: number | undefined
  onChange: (value: number | undefined) => void
  min?: number
  max?: number
}) {
  return (
    <label className="grid gap-1.5">
      <Label>{label}</Label>
      <Input
        type="number"
        value={value ?? ''}
        min={min}
        max={max}
        onChange={(event) => {
          const next = event.target.value
          onChange(next === '' ? undefined : Number(next))
        }}
      />
    </label>
  )
}

function SelectField({
  label,
  value,
  options,
  onChange,
}: {
  label: string
  value: string
  options: readonly string[]
  onChange: (value: string) => void
}) {
  return (
    <label className="grid gap-1.5">
      <Label>{label}</Label>
      <select
        value={value}
        onChange={(event) => onChange(event.target.value)}
        className="flex h-11 w-full rounded-xl border border-input bg-background/80 px-3 py-2 text-sm shadow-[inset_0_1px_0_rgba(255,255,255,0.5)] outline-none focus-visible:ring-2 focus-visible:ring-ring/60"
      >
        <option value="">None</option>
        {options.map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>
    </label>
  )
}

function buildCanvasBackground(spec: DmgSpec) {
  if (spec.background) {
    return `linear-gradient(180deg, rgba(255,255,255,0.78), rgba(255,255,255,0.48)), url("${spec.background}") center/cover no-repeat`
  }

  if (spec.backgroundColor) {
    return spec.backgroundColor
  }

  return 'linear-gradient(180deg, rgba(255,255,255,0.96), rgba(227,236,255,0.84))'
}

function getCanvasWidth(spec: DmgSpec) {
  return spec.window?.size?.width ?? 640
}

function getCanvasHeight(spec: DmgSpec) {
  return spec.window?.size?.height ?? 420
}

function getTypeGlyph(type: DmgContentType) {
  if (type === 'link') {
    return '⇢'
  }
  if (type === 'position') {
    return '◎'
  }
  return '▣'
}

function inferLabel(item: DmgContentItem) {
  const segments = item.path.split('/')
  return segments.at(-1) || item.path
}

function clamp(value: number, min: number, max: number) {
  return Math.max(min, Math.min(max, value))
}

function computeSnappedPosition(
  point: { x: number; y: number },
  spec: DmgSpec,
  movingId: string,
  snapToGrid: boolean,
) {
  const guides: GuideLine[] = []
  let x = point.x
  let y = point.y
  const width = getCanvasWidth(spec)
  const height = getCanvasHeight(spec)

  if (snapToGrid) {
    x = Math.round(x / GRID_SIZE) * GRID_SIZE
    y = Math.round(y / GRID_SIZE) * GRID_SIZE
  }

  const centerX = width / 2
  const centerY = height / 2

  if (Math.abs(x - centerX) <= GUIDE_THRESHOLD) {
    x = centerX
    guides.push({ orientation: 'vertical', position: centerX })
  }

  if (Math.abs(y - centerY) <= GUIDE_THRESHOLD) {
    y = centerY
    guides.push({ orientation: 'horizontal', position: centerY })
  }

  for (const item of spec.contents) {
    if (item.id === movingId) {
      continue
    }

    if (Math.abs(x - item.x) <= GUIDE_THRESHOLD) {
      x = item.x
      guides.push({ orientation: 'vertical', position: item.x })
    }
    if (Math.abs(y - item.y) <= GUIDE_THRESHOLD) {
      y = item.y
      guides.push({ orientation: 'horizontal', position: item.y })
    }
  }

  return {
    x: clamp(Math.round(x), CANVAS_PADDING, width - CANVAS_PADDING),
    y: clamp(Math.round(y), CANVAS_PADDING, height - CANVAS_PADDING),
    guides,
  }
}
