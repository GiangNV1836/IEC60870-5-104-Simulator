<script setup lang="ts">
import { useI18n } from '../i18n'

defineProps<{
  total: number
  selectedCount: number
}>()

defineEmits<{
  (event: 'select-all'): void
  (event: 'invert'): void
  (event: 'clear'): void
  (event: 'exit'): void
}>()

const { t } = useI18n()
</script>

<template>
  <div class="multi-select-actions">
    <button class="multi-select-btn" :disabled="total === 0" @click="$emit('select-all')">
      {{ t('table.selectFiltered') }}
    </button>
    <button class="multi-select-btn" :disabled="total === 0" @click="$emit('invert')">
      {{ t('table.invertFiltered') }}
    </button>
    <button class="multi-select-btn" :disabled="selectedCount === 0" @click="$emit('clear')">
      {{ t('table.clearSelection') }}
    </button>
    <span v-if="selectedCount > 0" class="multi-select-count">
      {{ t('table.selectedCount', { count: selectedCount }) }}
    </span>
    <button class="multi-select-btn exit" @click="$emit('exit')">
      {{ t('table.exitMultiSelect') }}
    </button>
  </div>
</template>

<style scoped>
.multi-select-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  white-space: nowrap;
}

.multi-select-btn {
  padding: 3px 6px;
  color: var(--c-overlay1);
  font-size: 10px;
  white-space: nowrap;
  cursor: pointer;
  background: var(--c-surface0);
  border: 1px solid var(--c-surface1);
  border-radius: 4px;
}

.multi-select-btn:hover:not(:disabled) {
  color: var(--c-text);
  background: var(--c-surface1);
}

.multi-select-btn:disabled {
  cursor: not-allowed;
  opacity: 0.4;
}

.multi-select-btn.exit {
  color: var(--c-peach);
}

.multi-select-count {
  color: var(--c-sapphire);
  font-size: 11px;
}
</style>
