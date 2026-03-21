import { createFileRoute } from '@tanstack/react-router'
import { DmgDesigner } from '../features/dmg-designer/DmgDesigner'

export const Route = createFileRoute('/dmg-designer')({
  head: () => ({
    meta: [
      {
        title: 'DMG Designer | Fastforge Studio',
      },
    ],
  }),
  component: DmgDesigner,
})
