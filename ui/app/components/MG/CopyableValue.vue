<template>
  <UInput
    :model-value="value"
    :ui="{ trailing: 'pr-0.5' }"
    readonly
  >
    <template #trailing>
      <UTooltip text="Copy to clipboard" :content="{ side: 'right' }">
        <UButton
          :color="copied ? 'success' : 'neutral'"
          variant="link"
          size="sm"
          :icon="copied ? 'i-lucide-copy-check' : 'i-lucide-copy'"
          aria-label="Copy to clipboard"
          @click="copy(value)"
        />
      </UTooltip>
    </template>
  </UInput>
</template>

<script setup lang="ts">
  import {useClipboard} from "@vueuse/core"

  const props = defineProps<{
    value: string,
  }>()

  const value = toRef(props.value)

  const { copy, copied } = useClipboard()
</script>

<style scoped>

</style>