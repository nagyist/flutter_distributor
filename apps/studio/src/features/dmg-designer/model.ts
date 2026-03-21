export const DMG_FORMATS = [
  'UDRW',
  'UDRO',
  'UDCO',
  'UDZO',
  'ULFO',
  'ULMO',
  'UDBZ',
] as const

export const DMG_FILESYSTEMS = ['HFS+', 'APFS'] as const

export const DMG_CONTENT_TYPES = ['link', 'file', 'position'] as const

export type DmgFormat = (typeof DMG_FORMATS)[number]
export type DmgFilesystem = (typeof DMG_FILESYSTEMS)[number]
export type DmgContentType = (typeof DMG_CONTENT_TYPES)[number]

export type DmgContentItem = {
  id: string
  x: number
  y: number
  type: DmgContentType
  path: string
  name?: string
}

export type DmgSpec = {
  title: string
  icon?: string
  background?: string
  backgroundColor?: string
  iconSize?: number
  format?: DmgFormat
  filesystem?: DmgFilesystem
  window?: {
    position?: {
      x: number
      y: number
    }
    size?: {
      width: number
      height: number
    }
  }
  contents: DmgContentItem[]
  codeSign?: {
    signingIdentity: string
    identifier?: string
  }
}

export type DmgValidationError = {
  field: string
  message: string
}

export type ValidationResult = {
  isValid: boolean
  errors: DmgValidationError[]
}

let idCounter = 0

export function createContentItem(
  overrides: Partial<Omit<DmgContentItem, 'id'>> = {},
): DmgContentItem {
  idCounter += 1

  return {
    id: `content-${idCounter}`,
    x: overrides.x ?? 192,
    y: overrides.y ?? 344,
    type: overrides.type ?? 'file',
    path: overrides.path ?? 'MyApp.app',
    name: overrides.name,
  }
}

export function createDefaultDmgSpec(): DmgSpec {
  return {
    title: 'My App',
    iconSize: 128,
    contents: [
      createContentItem({ type: 'file', path: 'MyApp.app', x: 192, y: 344 }),
      createContentItem({
        type: 'link',
        path: '/Applications',
        x: 448,
        y: 344,
        name: 'Applications',
      }),
    ],
  }
}

export function cloneSpec(spec: DmgSpec): DmgSpec {
  return {
    ...spec,
    window: spec.window
      ? {
          ...spec.window,
          position: spec.window.position
            ? { ...spec.window.position }
            : undefined,
          size: spec.window.size ? { ...spec.window.size } : undefined,
        }
      : undefined,
    contents: spec.contents.map((item) => ({ ...item })),
    codeSign: spec.codeSign ? { ...spec.codeSign } : undefined,
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

function readString(
  value: unknown,
  field: string,
  errors: DmgValidationError[],
  options: { required?: boolean } = {},
): string | undefined {
  if (value == null || value === '') {
    if (options.required) {
      errors.push({ field, message: 'This field is required.' })
    }
    return undefined
  }

  if (typeof value !== 'string') {
    errors.push({ field, message: 'Expected a string.' })
    return undefined
  }

  return value
}

function readNumber(
  value: unknown,
  field: string,
  errors: DmgValidationError[],
): number | undefined {
  if (value == null || value === '') {
    return undefined
  }

  if (typeof value !== 'number' || Number.isNaN(value)) {
    errors.push({ field, message: 'Expected a number.' })
    return undefined
  }

  return value
}

function readEnum<T extends readonly string[]>(
  value: unknown,
  field: string,
  allowed: T,
  errors: DmgValidationError[],
): T[number] | undefined {
  if (value == null || value === '') {
    return undefined
  }

  if (typeof value !== 'string' || !allowed.includes(value)) {
    errors.push({
      field,
      message: `Expected one of: ${allowed.join(', ')}.`,
    })
    return undefined
  }

  return value as T[number]
}

export function normalizeDmgSpec(input: unknown): {
  spec: DmgSpec
  errors: DmgValidationError[]
} {
  const errors: DmgValidationError[] = []
  const source = isRecord(input) ? input : {}

  const title = readString(source.title, 'title', errors, { required: true }) ?? ''
  const icon = readString(source.icon, 'icon', errors)
  const background = readString(source.background, 'background', errors)
  const backgroundColor = readString(
    source['background-color'] ?? source.backgroundColor,
    'background-color',
    errors,
  )
  const iconSize = readNumber(source['icon-size'] ?? source.iconSize, 'icon-size', errors)
  const format = readEnum(source.format, 'format', DMG_FORMATS, errors)
  const filesystem = readEnum(
    source.filesystem,
    'filesystem',
    DMG_FILESYSTEMS,
    errors,
  )

  let window: DmgSpec['window']
  if (source.window != null) {
    if (!isRecord(source.window)) {
      errors.push({ field: 'window', message: 'Expected an object.' })
    } else {
      const position = isRecord(source.window.position)
        ? {
            x: readNumber(source.window.position.x, 'window.position.x', errors) ?? 0,
            y: readNumber(source.window.position.y, 'window.position.y', errors) ?? 0,
          }
        : undefined
      const size = isRecord(source.window.size)
        ? {
            width:
              readNumber(source.window.size.width, 'window.size.width', errors) ?? 640,
            height:
              readNumber(source.window.size.height, 'window.size.height', errors) ?? 420,
          }
        : undefined
      if (source.window.position != null && !isRecord(source.window.position)) {
        errors.push({
          field: 'window.position',
          message: 'Expected an object.',
        })
      }
      if (source.window.size != null && !isRecord(source.window.size)) {
        errors.push({ field: 'window.size', message: 'Expected an object.' })
      }
      if (position || size) {
        window = { position, size }
      }
    }
  }

  let codeSign: DmgSpec['codeSign']
  const sourceCodeSign = source['code-sign'] ?? source.codeSign
  if (sourceCodeSign != null) {
    if (!isRecord(sourceCodeSign)) {
      errors.push({ field: 'code-sign', message: 'Expected an object.' })
    } else {
      const signingIdentity = readString(
        sourceCodeSign['signing-identity'] ?? sourceCodeSign.signingIdentity,
        'code-sign.signing-identity',
        errors,
        { required: true },
      )
      const identifier = readString(
        sourceCodeSign.identifier,
        'code-sign.identifier',
        errors,
      )

      if (signingIdentity) {
        codeSign = { signingIdentity, identifier }
      }
    }
  }

  const rawContents = source.contents
  const contents: DmgContentItem[] = Array.isArray(rawContents)
    ? rawContents.map((item, index) => {
        if (!isRecord(item)) {
          errors.push({
            field: `contents.${index}`,
            message: 'Expected an object.',
          })
          return createContentItem()
        }

        return createContentItem({
          x: readNumber(item.x, `contents.${index}.x`, errors) ?? 0,
          y: readNumber(item.y, `contents.${index}.y`, errors) ?? 0,
          type:
            readEnum(
              item.type,
              `contents.${index}.type`,
              DMG_CONTENT_TYPES,
              errors,
            ) ?? 'file',
          path:
            readString(item.path, `contents.${index}.path`, errors, {
              required: true,
            }) ?? '',
          name: readString(item.name, `contents.${index}.name`, errors),
        })
      })
    : []

  if (!Array.isArray(rawContents)) {
    errors.push({ field: 'contents', message: 'Expected a list.' })
  }

  const spec: DmgSpec = {
    title,
    icon,
    background,
    backgroundColor,
    iconSize,
    format,
    filesystem,
    window,
    contents,
    codeSign,
  }

  return { spec, errors }
}

export function validateDmgSpec(spec: DmgSpec): ValidationResult {
  const errors: DmgValidationError[] = []

  if (!spec.title.trim()) {
    errors.push({ field: 'title', message: '`title` must not be empty.' })
  }

  if (spec.contents.length === 0) {
    errors.push({ field: 'contents', message: '`contents` must not be empty.' })
  }

  if (spec.format && !DMG_FORMATS.includes(spec.format)) {
    errors.push({
      field: 'format',
      message: `Invalid format. Allowed: ${DMG_FORMATS.join(', ')}.`,
    })
  }

  if (spec.filesystem && !DMG_FILESYSTEMS.includes(spec.filesystem)) {
    errors.push({
      field: 'filesystem',
      message: `Invalid filesystem. Allowed: ${DMG_FILESYSTEMS.join(', ')}.`,
    })
  }

  for (const [index, item] of spec.contents.entries()) {
    if (!DMG_CONTENT_TYPES.includes(item.type)) {
      errors.push({
        field: `contents.${index}.type`,
        message: `Invalid content type. Allowed: ${DMG_CONTENT_TYPES.join(', ')}.`,
      })
    }

    if (!item.path.trim()) {
      errors.push({
        field: `contents.${index}.path`,
        message: 'Path is required.',
      })
    }
  }

  if (spec.codeSign && !spec.codeSign.signingIdentity.trim()) {
    errors.push({
      field: 'code-sign.signing-identity',
      message: 'Signing identity is required when code signing is enabled.',
    })
  }

  return {
    isValid: errors.length === 0,
    errors,
  }
}

export function toSerializableSpec(spec: DmgSpec): Record<string, unknown> {
  return {
    title: spec.title,
    icon: spec.icon,
    background: spec.background,
    'background-color': spec.backgroundColor,
    'icon-size': spec.iconSize,
    format: spec.format,
    filesystem: spec.filesystem,
    window: spec.window
      ? {
          position: spec.window.position,
          size: spec.window.size,
        }
      : undefined,
    contents: spec.contents.map(({ id: _id, ...item }) => item),
    'code-sign': spec.codeSign
      ? {
          'signing-identity': spec.codeSign.signingIdentity,
          identifier: spec.codeSign.identifier,
        }
      : undefined,
  }
}

export function pruneEmptyValues(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map((item) => pruneEmptyValues(item))
  }

  if (!isRecord(value)) {
    return value
  }

  const entries = Object.entries(value)
    .map(([key, entry]) => [key, pruneEmptyValues(entry)] as const)
    .filter(([, entry]) => {
      if (entry == null || entry === '') {
        return false
      }

      if (Array.isArray(entry)) {
        return true
      }

      if (isRecord(entry)) {
        return Object.keys(entry).length > 0
      }

      return true
    })

  return Object.fromEntries(entries)
}
