import { parse, stringify } from 'yaml'
import {
  type DmgSpec,
  type DmgValidationError,
  normalizeDmgSpec,
  pruneEmptyValues,
  toSerializableSpec,
  validateDmgSpec,
} from './model'

export type ParsedYamlResult =
  | {
      ok: true
      spec: DmgSpec
      errors: DmgValidationError[]
    }
  | {
      ok: false
      message: string
    }

export function specToYaml(spec: DmgSpec): string {
  const serializable = pruneEmptyValues(toSerializableSpec(spec))
  return stringify(serializable, {
    indent: 2,
    lineWidth: 0,
  })
}

export function yamlToSpec(input: string): ParsedYamlResult {
  try {
    const parsed = parse(input)
    const normalized = normalizeDmgSpec(parsed)
    const validation = validateDmgSpec(normalized.spec)

    return {
      ok: true,
      spec: normalized.spec,
      errors: [...normalized.errors, ...validation.errors],
    }
  } catch (error) {
    return {
      ok: false,
      message: error instanceof Error ? error.message : 'Failed to parse YAML.',
    }
  }
}
