import { beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { ref } from 'vue'
import { dialogKey } from '@shared/composables/useDialog'
import { useI18n } from '@shared/i18n'
import ConnectionTree from '../src/components/ConnectionTree.vue'

const invokeMock = vi.fn()
const alertMock = vi.fn(() => Promise.resolve())
const confirmMock = vi.fn(() => Promise.resolve(true))
const promptMock = vi.fn()

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}))

function mountTree(state = 'Stopped') {
  invokeMock.mockImplementation((command: string) => {
    if (command === 'list_servers') {
      return Promise.resolve([{
        id: 'server_1',
        bind_address: '0.0.0.0',
        port: 2404,
        state,
        station_count: 1,
        client_count: 2,
      }])
    }
    if (command === 'list_stations') {
      return Promise.resolve([{
        common_address: 456,
        name: '220TVAA',
        point_count: 3,
      }])
    }
    if (command === 'list_client_connections') {
      return Promise.resolve([
        { peer_address: '10.0.0.2:51001', data_transfer_active: true },
        { peer_address: '10.0.0.3:51002', data_transfer_active: false },
      ])
    }
    return Promise.resolve(undefined)
  })

  return mount(ConnectionTree, {
    global: {
      stubs: { Teleport: true, Transition: false },
      provide: {
        [dialogKey as symbol]: {
          showAlert: alertMock,
          showConfirm: confirmMock,
          showPrompt: promptMock,
        },
        treeRefreshKey: ref(0),
        selectedServerId: ref<string | null>(null),
        selectedCA: ref<number | null>(null),
        selectedCategory: ref<string | null>(null),
        categoryCounts: ref(new Map<string, number>()),
      },
    },
  })
}

describe('ConnectionTree', () => {
  beforeEach(() => {
    invokeMock.mockReset()
    alertMock.mockClear()
    confirmMock.mockClear()
    promptMock.mockReset()
    useI18n().setLocale('en-US')
  })

  it('always shows station name with CA and updates both fields while stopped', async () => {
    const wrapper = mountTree()
    await flushPromises()
    expect(wrapper.find('.station-node .node-label').text()).toBe('220TVAA (CA:456)')

    promptMock.mockResolvedValueOnce('457').mockResolvedValueOnce('New Name')
    await wrapper.find('.station-node').trigger('contextmenu')
    const editItem = wrapper.findAll('.context-menu-item').find(item => item.text() === 'Edit Station')
    expect(editItem).toBeDefined()
    await editItem!.trigger('click')
    await flushPromises()

    expect(invokeMock).toHaveBeenCalledWith('update_station', {
      request: {
        server_id: 'server_1',
        current_common_address: 456,
        common_address: 457,
        name: 'New Name',
      },
    })
    expect(wrapper.emitted('station-select')?.at(-1)).toEqual([
      'server_1', 457, 'Stopped', 'New Name',
    ])
    wrapper.unmount()
  })

  it('blocks CA changes while running before calling the backend', async () => {
    const wrapper = mountTree('Running')
    await flushPromises()
    promptMock.mockResolvedValueOnce('457')
    await wrapper.find('.station-node').trigger('contextmenu')
    const editItem = wrapper.findAll('.context-menu-item').find(item => item.text() === 'Edit Station')
    await editItem!.trigger('click')
    await flushPromises()

    expect(alertMock).toHaveBeenCalledWith(
      'Stop the server before changing the common address. The station name can be changed while running.',
    )
    expect(invokeMock.mock.calls.some(([command]) => command === 'update_station')).toBe(false)
    wrapper.unmount()
  })

  it('shows the connected-Master count and opens the live connection list', async () => {
    const wrapper = mountTree('Running')
    await flushPromises()

    const badge = wrapper.find('.client-count-badge')
    expect(badge.text()).toBe('2')
    expect(badge.attributes('title')).toBe('2 Master connection(s)')

    await badge.trigger('click')
    await flushPromises()
    expect(invokeMock).toHaveBeenCalledWith('list_client_connections', { serverId: 'server_1' })
    expect(wrapper.find('.connections-modal').exists()).toBe(true)
    expect(wrapper.findAll('.peer-address').map(cell => cell.text())).toEqual([
      '10.0.0.2:51001',
      '10.0.0.3:51002',
    ])
    wrapper.unmount()
  })
})
