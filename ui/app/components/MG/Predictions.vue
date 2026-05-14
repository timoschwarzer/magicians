<template>
  <div>
    <div class="grid auto-cols-auto gap-x-4">
      <template v-for="(player, index) in game.players">
        <div v-if="highlightIndex === index" class="col-start-1 flex items-center gap-1 text-xl">
          <UIcon name="i-lucide-arrow-right" />
        </div>
        <div class="col-start-2 underline-offset-2 decoration-1">{{ player.name }}</div>
        <div class="col-start-3 flex items-center gap-1">
          <UIcon name="i-lucide-crown" />
          {{ player.prediction ?? "-" }}
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"

  const props = defineProps<{
    game: MG.Game,
    highlightIndex?: number,
  }>()
</script>

<style scoped>

</style>