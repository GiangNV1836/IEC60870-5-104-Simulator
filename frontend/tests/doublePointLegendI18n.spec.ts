import { beforeEach, describe, expect, it } from 'vitest'
import { mount } from '@vue/test-utils'
import { nextTick } from 'vue'
import { useI18n } from '@shared/i18n'
import DoublePointLegend from '@shared/components/DoublePointLegend.vue'

describe('DoublePointLegend localization', () => {
  beforeEach(() => {
    useI18n().setLocale('zh-CN')
  })

  it('shows the same DPI explanation in the slave Value header', async () => {
    const wrapper = mount(DoublePointLegend, {
      global: { stubs: { Teleport: true } },
    })

    await wrapper.find('.dp-help').trigger('click')
    await nextTick()
    expect(wrapper.find('.dp-legend-title').text()).toBe('双点遥信 DPI · 双位置状态')
    expect(wrapper.text()).toContain('DPI 1')
    expect(wrapper.text()).toContain('分闸')
    expect(wrapper.text()).toContain('DPI 2')
    expect(wrapper.text()).toContain('合闸')
  })
})
