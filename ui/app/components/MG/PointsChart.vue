<template>
  <div class="chart-container" ref="container">
    <LineChart
      :data="chartData"
      :categories="chartCategories"
      :height="containerHeight"
    />
  </div>
</template>

<script lang="ts" setup>
  import type {MG} from '~/assets/script/mg.ts'
  import {useElementSize} from '@vueuse/core'

  const props = defineProps<{
    players: MG.Player[]
  }>()

  const container = useTemplateRef("container")
  const { height: containerHeight } = useElementSize(container)

  type DataPoint = {
    round: number,
    [key: string]: number,
  }

  const chartData = computed(() => {
    if (props.players.length === 0) {
      return []
    }

    const rounds = props.players[0]?.points?.length ?? 0
    const dataPoints: DataPoint[] = []

    for (let round = 0; round < rounds; round++) {
      const dataPoint: DataPoint = {round}

      for (let playerIndex = 0; playerIndex < props.players.length; playerIndex++) {
        dataPoint[String(playerIndex)] = props.players[playerIndex]!.points[round]!
      }

      dataPoints.push(dataPoint)
    }

    return dataPoints
  })

  const chartCategories = computed(() => {
    const categories: Record<string, BulletLegendItemInterface> = {}

    for (let playerIndex = 0; playerIndex < props.players.length; playerIndex++) {
      categories[String(playerIndex)] = {
        name: props.players[playerIndex]!.name,
      }
    }

    return categories
  })
</script>

<style scoped>

</style>
