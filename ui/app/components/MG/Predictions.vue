<template>
  <div>
    <div class="grid auto-cols-auto">
      <template v-for="(player, index) in game.players">
        <div class="col-start-1 flex items-center text-xl opacity-0 transition-opacity pr-3" :class="{'opacity-100': highlightIndex === index}">
          <UIcon name="i-lucide-arrow-right" />
        </div>
        <div class="col-start-2 underline-offset-2 decoration-1 pr-3">{{ player.name }}</div>
        <div class="col-start-3 flex items-center pr-1">
          <UIcon name="i-lucide-crown" />
        </div>
        <div class="col-start-4 relative text-center">
          <NumberFlow class="transition-opacity" :class="{'opacity-0': player.prediction === null}" :value="player.prediction ?? 0"/>
          <div class="text-center transition-opacity absolute inset-0" :class="{'opacity-0': player.prediction !== null}">-</div>
        </div>
      </template>

      <div class="col-start-4 mt-2 border-t -mx-1 text-center font-bold">
        <NumberFlow :value="predictionsSum"/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"
  import NumberFlow from "@number-flow/vue"

  const props = defineProps<{
    game: MG.Game,
    highlightIndex?: number,
  }>()

  const predictionsSum = computed(() => props.game.players.reduce((acc, player) => acc + (player.prediction ?? 0), 0))
  const anyPlayerPredicted = computed(() => props.game.players.some(player => player.prediction !== null))
</script>

<style scoped>

</style>