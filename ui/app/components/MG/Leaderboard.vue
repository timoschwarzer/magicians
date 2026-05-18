<template>
  <div>
    <div class="grid auto-cols-auto">
      <template v-for="player in sortedPlayers">
        <div v-if="giveCardsIndex === player.index" class="col-start-1 flex items-center text-xl pr-3">
          <UIcon name="i-tabler-cards" />
        </div>
        <div class="col-start-2 underline-offset-2 decoration-1 pr-3" :class="{'underline': highlightIndex === player.index}">{{ player.name }}</div>
        <MGPointsView class="col-start-3" :points="player.points" />
        <div
          v-if="player.pointsDelta !== null && game.state.type !== 'Setup' && game.state.round > 2"
          class="col-start-4 text-right text-xs self-end pl-2 opacity-60"
          :class="{'text-green': player.pointsDelta > 0, 'text-red': player.pointsDelta < 0}"
        >
          (<NumberFlow :prefix="player.pointsDelta >= 0 ? '+' : ''" class="col-start-4" :value="player.pointsDelta"/>)
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"
  import NumberFlow from "@number-flow/vue"

  const props = defineProps<{
    game: MG.Game,
    highlightIndex?: number,
    giveCardsIndex?: number,
  }>()

  const sortedPlayers = computed(() => {
    return props.game.players.map((p, index) => ({...p, index})).toSorted((a, b) => b.points - a.points)
  })
</script>

<style scoped>

</style>