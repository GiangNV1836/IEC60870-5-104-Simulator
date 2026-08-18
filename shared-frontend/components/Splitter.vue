<script setup lang="ts">
import { onBeforeUnmount } from 'vue'

const props = defineProps<{
  modelValue: number
  axis: 'x' | 'y'
  min: number
  max: number
  reverse?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: number): void
}>()

let startValue = 0
let startPosition = 0

function onMouseDown(event: MouseEvent) {
  event.preventDefault()
  startValue = props.modelValue
  startPosition = props.axis === 'x' ? event.clientX : event.clientY
  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
  document.body.style.userSelect = 'none'
  document.body.style.cursor = props.axis === 'x' ? 'col-resize' : 'row-resize'
}

function onMouseMove(event: MouseEvent) {
  const currentPosition = props.axis === 'x' ? event.clientX : event.clientY
  let delta = currentPosition - startPosition
  if (props.reverse) delta = -delta
  emit('update:modelValue', Math.min(props.max, Math.max(props.min, startValue + delta)))
}

function onMouseUp() {
  document.removeEventListener('mousemove', onMouseMove)
  document.removeEventListener('mouseup', onMouseUp)
  document.body.style.userSelect = ''
  document.body.style.cursor = ''
}

onBeforeUnmount(onMouseUp)
</script>

<template>
  <div
    :class="['splitter', `axis-${axis}`]"
    role="separator"
    :aria-orientation="axis === 'x' ? 'vertical' : 'horizontal'"
    @mousedown="onMouseDown"
  />
</template>

<style scoped>
.splitter {
  position: relative;
  z-index: 5;
  background: transparent;
}

.axis-x {
  width: 4px;
  height: 100%;
  cursor: col-resize;
}

.axis-y {
  width: 100%;
  height: 4px;
  cursor: row-resize;
}

.splitter::before {
  content: '';
  position: absolute;
  background: var(--c-surface0);
  transition: background 0.12s, transform 0.12s;
}

.axis-x::before {
  top: 0;
  bottom: 0;
  left: 1px;
  width: 1px;
}

.axis-y::before {
  right: 0;
  left: 0;
  top: 1px;
  height: 1px;
}

.splitter:hover::before,
.splitter:active::before {
  background: var(--c-blue);
}

.axis-x:hover::before,
.axis-x:active::before {
  left: 1px;
  width: 2px;
}

.axis-y:hover::before,
.axis-y:active::before {
  top: 1px;
  height: 2px;
}
</style>
