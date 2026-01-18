<template>
  <div class="space-y-4">
    <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden bg-white dark:bg-gray-800">
      <!-- Toolbar -->
      <div class="flex flex-wrap gap-1 p-3 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
        <button
          @click="toggleBold"
          :class="[
            'px-3 py-2 rounded text-sm font-medium transition-colors',
            isBold 
              ? 'bg-blue-500 text-white' 
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600'
          ]"
          type="button"
          title="Bold (Ctrl+B)"
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
          title="Italic (Ctrl+I)"
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
          title="Underline (Ctrl+U)"
        >
          <u>U</u>
        </button>
        <div class="w-px bg-gray-300 dark:bg-gray-600 mx-1"></div>
        <button
          @click="insertHeading"
          :class="[
            'px-3 py-2 rounded text-sm font-medium transition-colors',
            'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600'
          ]"
          type="button"
          title="Heading"
        >
          H
        </button>
      </div>

      <!-- Editor Area -->
      <div
        ref="editorContainer"
        @click="focusEditor"
        class="p-4 min-h-64 focus:outline-none text-gray-900 dark:text-gray-100"
      >
        <div v-if="!isEditing" class="text-gray-400 dark:text-gray-500">
          {{ $t('pages.components.lexical.placeholder') }}
        </div>
        <div v-else v-html="editorContent" class="whitespace-pre-wrap break-words"></div>
      </div>
    </div>

    <!-- Word Count -->
    <div class="flex justify-between items-center text-xs text-gray-500 dark:text-gray-400">
      <span>{{ $t('pages.components.lexical.characters') }}: {{ editorContent.length }}</span>
      <span>{{ $t('pages.components.lexical.words') }}: {{ wordCount }}</span>
    </div>

    <!-- Editor State Display -->
    <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
      <p class="text-xs text-gray-600 dark:text-gray-400 mb-2 font-semibold">
        {{ $t('pages.components.lexical.output') }}
      </p>
      <div class="text-xs text-gray-700 dark:text-gray-300 font-mono bg-white dark:bg-gray-800 p-2 rounded border border-gray-200 dark:border-gray-700 max-h-24 overflow-y-auto">
        {{ editorContent || $t('pages.components.lexical.noContent') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

const editorContainer = ref<HTMLDivElement | null>(null)
const editorContent = ref<string>('')
const isEditing = ref(false)
const isBold = ref(false)
const isItalic = ref(false)
const isUnderline = ref(false)

const { t } = useI18n()

const wordCount = computed(() => {
  if (!editorContent.value) return 0
  return editorContent.value
    .trim()
    .split(/\s+/)
    .filter(word => word.length > 0).length
})

onMounted(() => {
  if (editorContainer.value) {
    editorContainer.value.addEventListener('input', handleInput)
    editorContainer.value.addEventListener('focus', () => {
      isEditing.value = true
    })
    editorContainer.value.addEventListener('blur', () => {
      if (editorContent.value.trim() === '') {
        isEditing.value = false
      }
    })
  }
})

const handleInput = (event: Event) => {
  const target = event.target as HTMLElement
  editorContent.value = target.textContent || ''
  updateToolbarState()
}

const updateToolbarState = () => {
  const selection = window.getSelection()
  if (selection && selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    const fragment = range.cloneContents()
    
    // Check for bold
    isBold.value = fragment.querySelector('strong') !== null
    // Check for italic
    isItalic.value = fragment.querySelector('em') !== null
    // Check for underline
    isUnderline.value = fragment.querySelector('u') !== null
  }
}

const applyFormat = (tag: string) => {
  const selection = window.getSelection()
  if (selection && selection.rangeCount > 0 && selection.toString()) {
    const range = selection.getRangeAt(0)
    const element = document.createElement(tag)
    element.appendChild(range.extractContents())
    range.insertNode(element)
    updateToolbarState()
  }
}

const toggleBold = () => {
  applyFormat('strong')
}

const toggleItalic = () => {
  applyFormat('em')
}

const toggleUnderline = () => {
  applyFormat('u')
}

const insertHeading = () => {
  if (editorContainer.value) {
    const heading = document.createElement('h3')
    heading.textContent = t('pages.components.lexical.headingPlaceholder')
    heading.className = 'text-lg font-bold mt-4 mb-2'
    editorContainer.value.appendChild(document.createElement('br'))
    editorContainer.value.appendChild(heading)
    editorContainer.value.focus()
    editorContent.value = editorContainer.value.textContent || ''
  }
}

const focusEditor = () => {
  isEditing.value = true
}
</script>
