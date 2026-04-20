<template>
  <div>
    <div class="grid auto-cols-auto gap-x-4">
      <template v-for="player in sortedPlayers">
        <div v-if="giveCardsIndex === player.index" class="col-start-1 flex items-center gap-1 text-xl">
          <UIcon name="i-tabler-cards" />
        </div>
        <div class="col-start-2 underline-offset-2 decoration-1" :class="{'underline': highlightIndex === player.index}">{{ player.name }}</div>
        <MGPointsView class="col-start-3" :points="player.points" />
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"

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