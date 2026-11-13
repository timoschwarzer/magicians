<template>
  <div>
    <div class="grid auto-cols-auto">
      <template v-for="player in sortedPlayers">
        <div v-if="giveCardsIndex === player.index" class="col-start-1 flex items-center text-xl pr-3">
          <UIcon name="i-tabler-cards" />
        </div>
        <div v-if="startingIndex === player.index" class="col-start-1 flex items-center text-xl pr-3">
          <UIcon name="i-lucide-play" />
        </div>
        <div class="col-start-2 underline-offset-2 decoration-1 pr-3" :class="{'underline': highlightIndex === player.index}">{{ player.name }}</div>
        <MGPointsView class="col-start-3" :points="player.points.at(-1) ?? 0" />
        <div
          v-if="player.points.length >= 2 && game.state.type !== 'Setup'"
          class="col-start-4 text-right text-xs self-end pl-2 opacity-60"
          :class="pointsDeltaColorClass(player.points)"
        >
          (<NumberFlow :prefix="getPointsDelta(player.points) >= 0 ? '+' : ''" class="col-start-4" :value="getPointsDelta(player.points)"/>)
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
    startingIndex?: number,
  }>()

  const sortedPlayers = computed(() => {
    return props.game.players.map((p, index) => ({...p, index})).toSorted((a, b) => (b.points.at(-1) ?? 0) - (a.points.at(-1) ?? 0))
  })

  function getPointsDelta(points: number[]): number {
    if (points.length < 2) {
      return 0
    }

    const lastPoints = points.at(-1) ?? 0
    const secondToLastPoints = points.at(-2) ?? 0

    return lastPoints - secondToLastPoints
  }

  function pointsDeltaColorClass(points: number[]) {
    const pointsDelta = getPointsDelta(points)

    if (pointsDelta === null) {
      return ""
    }

    if (pointsDelta > 0) {
      return "text-green"
    }

    if (pointsDelta < 0) {
      return "text-red"
    }
  }
</script>

<style scoped>

</style>
