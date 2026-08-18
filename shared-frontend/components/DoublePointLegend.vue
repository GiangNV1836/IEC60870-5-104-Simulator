<script setup lang="ts">
import { ref, onBeforeUnmount } from 'vue'
import { useI18n } from '../i18n'

const { t } = useI18n()

const STATES = [
  { code: '0', key: 'intermediate' },
  { code: '1', key: 'off' },
  { code: '2', key: 'on' },
  { code: '3', key: 'indeterminate' },
] as const

const open = ref(false)
const btn = ref<HTMLElement | null>(null)
const pos = ref({ top: 0, right: 0 })

function place() {
  const el = btn.value
  if (!el) return
  const rect = el.getBoundingClientRect()
  pos.value = {
    top: rect.bottom + 4,
    right: Math.max(8, window.innerWidth - rect.right),
  }
}

function openLegend() {
  place()
  open.value = true
  requestAnimationFrame(() => {
    document.addEventListener('pointerdown', onDocumentPointer, true)
    document.addEventListener('keydown', onKeydown, true)
    window.addEventListener('scroll', close, true)
    window.addEventListener('resize', close, true)
  })
}

function close() {
  if (!open.value) return
  open.value = false
  document.removeEventListener('pointerdown', onDocumentPointer, true)
  document.removeEventListener('keydown', onKeydown, true)
  window.removeEventListener('scroll', close, true)
  window.removeEventListener('resize', close, true)
}

function toggle() {
  open.value ? close() : openLegend()
}

function onDocumentPointer(event: Event) {
  const target = event.target as HTMLElement
  if (btn.value?.contains(target)) return
  if (target.closest?.('.dp-legend')) return
  close()
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') close()
}

onBeforeUnmount(close)
</script>

<template>
  <button
    ref="btn"
    type="button"
    class="dp-help"
    :aria-label="t('doublePoint.legendTitle')"
    @click.stop="toggle"
  >?</button>
  <Teleport to="body">
    <div
      v-if="open"
      class="dp-legend"
      :style="{ top: pos.top + 'px', right: pos.right + 'px' }"
    >
      <div class="dp-legend-title">{{ t('doublePoint.legendTitle') }}</div>
      <div v-for="state in STATES" :key="state.code" class="dp-legend-row">
        <span class="dp-legend-code">DPI {{ state.code }}</span>
        <span class="dp-legend-token">{{ t(`doublePoint.tokens.${state.key}`) }}</span>
        <span class="dp-legend-desc">{{ t(`doublePoint.states.${state.key}`) }}</span>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.dp-help {
  width: 16px;
  height: 16px;
  line-height: 14px;
  border-radius: 50%;
  border: 1px solid var(--c-surface2);
  background: var(--c-surface0);
  color: var(--c-subtext0);
  font-size: 11px;
  cursor: pointer;
  padding: 0;
}

.dp-help:hover {
  border-color: var(--c-blue);
  color: var(--c-blue);
}

.dp-legend {
  position: fixed;
  z-index: 1000;
  min-width: 300px;
  padding: 8px 10px;
  background: var(--c-mantle);
  border: 1px solid var(--c-surface1);
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
}

.dp-legend-title {
  margin-bottom: 6px;
  color: var(--c-subtext1);
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
}

.dp-legend-row {
  display: grid;
  grid-template-columns: 44px 48px 1fr;
  gap: 6px;
  align-items: baseline;
  padding: 2px 0;
  font-size: 12px;
}

.dp-legend-code {
  color: var(--c-peach);
  font: 600 11px/1 var(--font-mono);
}

.dp-legend-token {
  color: var(--c-text);
  font: 600 12px/1 var(--font-mono);
}

.dp-legend-desc {
  color: var(--c-subtext0);
  font-size: 11px;
}
</style>
