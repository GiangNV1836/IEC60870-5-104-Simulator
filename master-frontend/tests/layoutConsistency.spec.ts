import { beforeEach, describe, expect, it, vi } from 'vitest'
import { mount, shallowMount } from '@vue/test-utils'
import { ref } from 'vue'
import App from '../src/App.vue'
import DataTable from '../src/components/DataTable.vue'
import ValuePanel from '../src/components/ValuePanel.vue'
import Splitter from '@shared/components/Splitter.vue'

vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn(() => Promise.resolve(() => {})) }))
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(() => Promise.resolve([])) }))

describe('Master/Slave layout consistency', () => {
  beforeEach(() => {
    localStorage.clear()
  })

  it('uses the shared persisted left, right, and log splitters', () => {
    localStorage.setItem('iec104master.layout.treeWidth', '320')
    localStorage.setItem('iec104master.layout.panelWidth', '360')
    localStorage.setItem('iec104.logPanel.height', '240')
    const wrapper = shallowMount(App)
    const vm = wrapper.vm as unknown as {
      treeWidth: number
      panelWidth: number
      logHeight: number
    }

    expect(vm.treeWidth).toBe(320)
    expect(vm.panelWidth).toBe(360)
    expect(vm.logHeight).toBe(240)
    expect(wrapper.findAllComponents(Splitter)).toHaveLength(3)
    expect(wrapper.find('.app-layout').attributes('style')).toContain('--tree-w: 320px')
    expect(wrapper.find('.app-layout').attributes('style')).toContain('--panel-w: 360px')
    wrapper.unmount()
  })

  it('uses guided empty states in the table and point-details panel', () => {
    const commonProvide = {
      selectedConnectionId: ref<string | null>(null),
      selectedCA: ref<number | null>(null),
      selectedCategory: ref<string | null>(null),
      selectedPoints: ref([]),
      dataRefreshKey: ref(0),
      changedCategories: ref(new Map()),
      categoryCounts: ref(new Map()),
    }
    const table = mount(DataTable, { global: { provide: commonProvide } })
    const panel = mount(ValuePanel, { global: { provide: commonProvide } })

    expect(table.find('.empty-state-title').text()).toBe('Select a connection to view data')
    expect(table.find('.empty-state-hint').text()).toContain('navigation tree')
    expect(panel.find('.panel-header').text()).toBe('Point Details')
    expect(panel.find('.empty-state-title').text()).toBe('No data point selected')
    expect(panel.find('.empty-state-hint').text()).toContain('Click any row')
    table.unmount()
    panel.unmount()
  })
})
