import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { useI18n } from '@shared/i18n'
import ClientConnectionsModal from '../src/components/ClientConnectionsModal.vue'
import type { ClientConnectionInfo } from '../src/types'

const invokeMock = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}))

describe('ClientConnectionsModal', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    invokeMock.mockReset()
    useI18n().setLocale('en-US')
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('shows each Master peer and refreshes STARTDT state and connection count', async () => {
    let rows: ClientConnectionInfo[] = [
      { peer_address: '10.0.0.2:51001', data_transfer_active: true },
      { peer_address: '10.0.0.3:51002', data_transfer_active: false },
    ]
    invokeMock.mockImplementation(() => Promise.resolve(rows))

    const wrapper = mount(ClientConnectionsModal, {
      props: {
        visible: true,
        serverId: 'server_1',
        serverLabel: '0.0.0.0:2404',
      },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await flushPromises()

    expect(invokeMock).toHaveBeenCalledWith('list_client_connections', { serverId: 'server_1' })
    expect(wrapper.find('.connections-summary').text()).toContain('2 Master connection(s)')
    expect(wrapper.findAll('.peer-address').map(cell => cell.text())).toEqual([
      '10.0.0.2:51001',
      '10.0.0.3:51002',
    ])
    expect(wrapper.findAll('.connection-state').map(cell => cell.text())).toEqual([
      'Data Transfer',
      'Connected',
    ])

    rows = [{ peer_address: '10.0.0.3:51002', data_transfer_active: true }]
    await vi.advanceTimersByTimeAsync(1000)
    await flushPromises()
    expect(wrapper.find('.connections-summary').text()).toContain('1 Master connection(s)')
    expect(wrapper.find('.connection-state').text()).toBe('Data Transfer')

    await wrapper.find('.connections-done').trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(1)
    wrapper.unmount()
  })
})
