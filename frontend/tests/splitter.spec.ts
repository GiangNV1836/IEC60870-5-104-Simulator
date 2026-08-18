import { describe, expect, it } from 'vitest'
import { mount } from '@vue/test-utils'
import Splitter from '@shared/components/Splitter.vue'

describe('shared Splitter', () => {
  it('drags, clamps, and supports reverse sizing', async () => {
    const wrapper = mount(Splitter, {
      props: { modelValue: 240, axis: 'x', min: 180, max: 480 },
    })
    await wrapper.trigger('mousedown', { clientX: 100 })
    document.dispatchEvent(new MouseEvent('mousemove', { clientX: 180 }))
    expect(wrapper.emitted('update:modelValue')?.at(-1)).toEqual([320])

    document.dispatchEvent(new MouseEvent('mousemove', { clientX: -1000 }))
    expect(wrapper.emitted('update:modelValue')?.at(-1)).toEqual([180])
    document.dispatchEvent(new MouseEvent('mouseup'))

    await wrapper.setProps({ modelValue: 280, reverse: true })
    await wrapper.trigger('mousedown', { clientX: 200 })
    document.dispatchEvent(new MouseEvent('mousemove', { clientX: 240 }))
    expect(wrapper.emitted('update:modelValue')?.at(-1)).toEqual([240])
    document.dispatchEvent(new MouseEvent('mouseup'))
    wrapper.unmount()
  })
})
