<template>
  <div class="space-y-4">
    <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden bg-white dark:bg-gray-800">
      <!-- Toolbar -->
      <div class="flex flex-wrap gap-2 p-3 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
        <!-- Basic formatting -->
        <button
          @click="toggleBold"
          :class="[
            'px-3 py-2 rounded text-sm font-medium transition-colors',
            isBold
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600'
          ]"
          type="button"
          :title="`${$t('pages.components.lexical.bold')} (Ctrl+B)`"
        >
          <strong>B</strong>
        </button>
        <button
          @click="toggleItalic"
          :class="[
            'px-3 py-2 rounded text-sm font-medium transition-colors',
            isItalic
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600'
          ]"
          type="button"
          :title="`${$t('pages.components.lexical.italic')} (Ctrl+I)`"
        >
          <em>I</em>
        </button>
        <button
          @click="toggleUnderline"
          :class="[
            'px-3 py-2 rounded text-sm font-medium transition-colors',
            isUnderline
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600'
          ]"
          type="button"
          :title="`${$t('pages.components.lexical.underline')} (Ctrl+U)`"
        >
          <u>U</u>
        </button>

        <div class="w-px bg-gray-300 dark:bg-gray-600 mx-1"></div>

        <!-- Heading select -->
        <select
          :value="currentHeading"
          @change="changeHeading"
          :class="[
            'px-3 py-2 rounded text-sm font-medium transition-colors',
            currentHeading !== 'normal'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 dark:bg-gray-700 dark:text-gray-300'
          ]"
        >
          <option value="normal">{{ $t('pages.components.lexical.normal') }}</option>
          <option value="h1">{{ $t('pages.components.lexical.heading1') }}</option>
          <option value="h2">{{ $t('pages.components.lexical.heading2') }}</option>
          <option value="h3">{{ $t('pages.components.lexical.heading3') }}</option>
          <option value="h4">{{ $t('pages.components.lexical.heading4') }}</option>
          <option value="h5">{{ $t('pages.components.lexical.heading5') }}</option>
        </select>

        <div class="w-px bg-gray-300 dark:bg-gray-600 mx-1"></div>

        <!-- Color pickers -->
        <div class="flex items-center gap-2">
          <label :title="$t('pages.components.lexical.textColor')" class="cursor-pointer">
            <input
              type="color"
              :value="textColor"
              @change="changeTextColor"
              class="w-8 h-8 rounded border border-gray-300 dark:border-gray-600 cursor-pointer"
            />
          </label>
          <label :title="$t('pages.components.lexical.backgroundColor')" class="cursor-pointer">
            <input
              type="color"
              :value="backgroundColor"
              @change="changeBackgroundColor"
              class="w-8 h-8 rounded border border-gray-300 dark:border-gray-600 cursor-pointer"
            />
          </label>
        </div>
      </div>

      <!-- Editor -->
      <div
        ref="editorContainer"
        class="editor-wrapper p-4 min-h-64 text-gray-900 dark:text-gray-100"
      >
        <div
          ref="editor"
          contenteditable="true"
          class="editor-content outline-none"
          :placeholder="$t('pages.components.lexical.placeholder')"
        >
          {{ $t('pages.components.lexical.placeholder') }}
        </div>
      </div>
    </div>

    <!-- Word Count -->
    <div class="flex justify-between items-center text-xs text-gray-500 dark:text-gray-400">
      <span>{{ $t('pages.components.lexical.characters') }}: {{ characterCount }}</span>
      <span>{{ $t('pages.components.lexical.words') }}: {{ wordCount }}</span>
    </div>

    <!-- Editor JSON State Display -->
    <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
      <p class="text-xs text-gray-600 dark:text-gray-400 mb-2 font-semibold">
        {{ $t('pages.components.lexical.output') }}
      </p>
      <div class="text-xs text-gray-700 dark:text-gray-300 font-mono bg-white dark:bg-gray-800 p-2 rounded border border-gray-200 dark:border-gray-700 max-h-32 overflow-y-auto whitespace-pre-wrap">
        {{ editorHtml || $t('pages.components.lexical.noContent') }}
      </div>
    </div>

    <!-- Lexical Documentation Link -->
    <div class="p-4 bg-blue-50 dark:bg-blue-900 border border-blue-200 dark:border-blue-800 rounded-lg">
      <p class="text-sm text-blue-900 dark:text-blue-200">
        <strong>{{ $t('pages.components.lexical.learnMore') }}:</strong>
        <a href="https://github.com/facebook/lexical" target="_blank" class="underline hover:opacity-75 font-semibold">
          github.com/facebook/lexical
        </a>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

const editorContainer = ref<HTMLDivElement | null>(null)
const editor = ref<HTMLDivElement | null>(null)
const editorHtml = ref<string>('')
const isBold = ref(false)
const isItalic = ref(false)
const isUnderline = ref(false)
const currentHeading = ref<string>('normal')
const textColor = ref<string>('#000000')
const backgroundColor = ref<string>('#ffffff')

const { t } = useI18n()

const characterCount = computed(() => {
  if (!editor.value) return 0
  return editor.value.textContent?.length || 0
})

const wordCount = computed(() => {
  if (!editor.value) return 0
  const text = editor.value.textContent || ''
  return text
    .trim()
    .split(/\s+/)
    .filter(word => word.length > 0).length
})

onMounted(() => {
  if (editor.value) {
    // Clear placeholder on first input
    if (editor.value.textContent === t('pages.components.lexical.placeholder')) {
      editor.value.textContent = ''
    }

    editor.value.addEventListener('input', updateState)
    editor.value.addEventListener('mouseup', updateToolbarState)
    editor.value.addEventListener('keyup', updateToolbarState)
  }
})

onUnmounted(() => {
  if (editor.value) {
    editor.value.removeEventListener('input', updateState)
    editor.value.removeEventListener('mouseup', updateToolbarState)
    editor.value.removeEventListener('keyup', updateToolbarState)
  }
})

const updateState = () => {
  if (editor.value) {
    editorHtml.value = editor.value.innerHTML
  }
}

const updateToolbarState = () => {
  if (!editor.value) return

  const selection = window.getSelection()
  if (!selection || selection.rangeCount === 0) {
    isBold.value = false
    isItalic.value = false
    isUnderline.value = false
    currentHeading.value = 'normal'
    return
  }

  // Check if any selected text has formatting
  const range = selection.getRangeAt(0)
  let node = range.commonAncestorContainer

  if (node.nodeType === Node.TEXT_NODE) {
    node = node.parentElement as Node
  }

  // Walk up the tree to check for formatting
  let parent = node as Element | null
  isBold.value = false
  isItalic.value = false
  isUnderline.value = false
  currentHeading.value = 'normal'

  while (parent && parent !== editor.value) {
    const tag = parent.tagName?.toLowerCase()
    if (tag === 'strong' || tag === 'b') isBold.value = true
    if (tag === 'em' || tag === 'i') isItalic.value = true
    if (tag === 'u') isUnderline.value = true
    if (tag === 'h1') currentHeading.value = 'h1'
    if (tag === 'h2') currentHeading.value = 'h2'
    if (tag === 'h3') currentHeading.value = 'h3'
    if (tag === 'h4') currentHeading.value = 'h4'
    if (tag === 'h5') currentHeading.value = 'h5'
    parent = parent.parentElement
  }

  // Get current text color and background color
  if (parent) {
    const computedStyle = window.getComputedStyle(parent)
    const color = computedStyle.color
    textColor.value = rgbToHex(color)
    const bgColor = computedStyle.backgroundColor
    backgroundColor.value = rgbToHex(bgColor)
  }
}

const rgbToHex = (rgb: string): string => {
  const match = rgb.match(/^rgb\((\d+),\s*(\d+),\s*(\d+)\)$/)
  if (!match) return '#000000'

  const hex = (x: string) => {
    const hex = parseInt(x).toString(16)
    return hex.length === 1 ? '0' + hex : hex
  }

  return '#' + hex(match[1]) + hex(match[2]) + hex(match[3])
}

const execCommand = (command: string, value?: string) => {
  document.execCommand(command, false, value)
  updateState()
  updateToolbarState()
  editor.value?.focus()
}

const toggleBold = () => {
  execCommand('bold')
}

const toggleItalic = () => {
  execCommand('italic')
}

const toggleUnderline = () => {
  execCommand('underline')
}

const changeHeading = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  if (!editor.value) return

  // Get current selection
  const selection = window.getSelection()
  if (!selection || selection.rangeCount === 0) return

  if (value === 'normal') {
    // Remove heading format - wrap in paragraph
    document.execCommand('formatBlock', false, 'p')
  } else {
    // Apply heading format
    document.execCommand('formatBlock', false, value)
  }

  updateState()
  updateToolbarState()
  editor.value.focus()
}

const changeTextColor = (event: Event) => {
  const color = (event.target as HTMLInputElement).value
  execCommand('foreColor', color)
}

const changeBackgroundColor = (event: Event) => {
  const color = (event.target as HTMLInputElement).value
  execCommand('backColor', color)
}
</script>

<style scoped>
/* Override global heading styles inside the editor */
:deep(.editor-content h1) {
  font-size: 2em;
  font-weight: bold;
  margin: 0.67em 0;
  display: block;
  color: inherit;
}

:deep(.editor-content h2) {
  font-size: 1.5em;
  font-weight: bold;
  margin: 0.75em 0;
  display: block;
  color: inherit;
}

:deep(.editor-content h3) {
  font-size: 1.17em;
  font-weight: bold;
  margin: 0.83em 0;
  display: block;
  color: inherit;
}

:deep(.editor-content h4) {
  font-weight: bold;
  margin: 1em 0;
  display: block;
  color: inherit;
}

:deep(.editor-content h5) {
  font-size: 0.83em;
  font-weight: bold;
  margin: 1.17em 0;
  display: block;
  color: inherit;
}

:deep(.editor-content h6) {
  font-size: 0.67em;
  font-weight: bold;
  margin: 1.33em 0;
  display: block;
  color: inherit;
}

/* Ensure paragraphs are properly styled */
:deep(.editor-content p) {
  margin: 1em 0;
  display: block;
  color: inherit;
}

/* Default text formatting */
:deep(.editor-content b),
:deep(.editor-content strong) {
  font-weight: bold;
}

:deep(.editor-content i),
:deep(.editor-content em) {
  font-style: italic;
}

:deep(.editor-content u) {
  text-decoration: underline;
}

/* Preserve line height and text flow */
:deep(.editor-content) {
  line-height: 1.6;
}
</style>
