import { describe, expect, it } from 'vitest'
import {
  createDefaultDmgSpec,
  normalizeDmgSpec,
  validateDmgSpec,
} from './model'
import { specToYaml, yamlToSpec } from './yaml'

describe('dmg designer model', () => {
  it('creates a valid default spec', () => {
    const spec = createDefaultDmgSpec()
    const result = validateDmgSpec(spec)

    expect(result.isValid).toBe(true)
    expect(spec.contents).toHaveLength(2)
  })

  it('serializes and parses YAML round-trip', () => {
    const spec = createDefaultDmgSpec()
    const yaml = specToYaml({
      ...spec,
      backgroundColor: '#123456',
      window: {
        position: { x: 120, y: 160 },
        size: { width: 720, height: 480 },
      },
      codeSign: {
        signingIdentity: 'Developer ID Application: Fastforge',
        identifier: 'dev.fastforge.demo',
      },
    })

    const parsed = yamlToSpec(yaml)

    expect(parsed.ok).toBe(true)
    if (parsed.ok) {
      expect(parsed.errors).toHaveLength(0)
      expect(parsed.spec.title).toBe('My App')
      expect(parsed.spec.backgroundColor).toBe('#123456')
      expect(parsed.spec.window?.size?.width).toBe(720)
      expect(parsed.spec.codeSign?.identifier).toBe('dev.fastforge.demo')
      expect(parsed.spec.contents).toHaveLength(2)
    }
  })

  it('reports invalid enum values during normalization and validation', () => {
    const result = normalizeDmgSpec({
      title: '',
      format: 'BAD',
      filesystem: 'EXT4',
      contents: [
        {
          x: 10,
          y: 20,
          type: 'weird',
          path: '',
        },
      ],
      'code-sign': {
        'signing-identity': '',
      },
    })

    const validation = validateDmgSpec(result.spec)

    expect(result.errors.some((error) => error.field === 'format')).toBe(true)
    expect(
      result.errors.some((error) => error.field === 'contents.0.type'),
    ).toBe(true)
    expect(validation.errors.some((error) => error.field === 'title')).toBe(true)
    expect(
      result.errors.some(
        (error) => error.field === 'code-sign.signing-identity',
      ),
    ).toBe(true)
  })
})
