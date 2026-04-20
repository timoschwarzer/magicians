<template>
  <div class="flex gap-2 items-center">
    <div>{{ player.name }}</div>
    <MGPointsView :points="player.points" />
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"

  defineProps<{
    player: MG.Player,
  }>()
</script>

<style scoped>

</style>