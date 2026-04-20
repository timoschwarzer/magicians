<template>
  <UButton
    :disabled="disabled"
    size="xl"
    :color="ready ? 'success' : undefined"
    :icon="ready ? 'i-lucide-check' : 'i-lucide-square'"
    @click="onButtonClick"
  >
    Ready
  </UButton>
</template>

<script setup lang="ts">
  const props = defineProps<{
    ready: boolean,
    disabled?: boolean,
  }>()

  const emits = defineEmits<{
    (e: "signalReady", ready: boolean): void
  }>()

  function onButtonClick() {
    emits("signalReady", !props.ready)
  }
</script>

<style scoped>

</style>