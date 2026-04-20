<template>
  <div class="flex gap-0.5 items-center justify-between">
    <UIcon name="i-lucide-coins" class="bg-blue-400"/>
    <div>
      <NumberFlow :value="points"/>
    </div>
  </div>
</template>

<script setup lang="ts">
  import NumberFlow from "@number-flow/vue"

  defineProps<{
    points: number,
  }>()
</script>

<style scoped>

</style>