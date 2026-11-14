<template>
  <div class="chart-container" ref="container">
    <UChart
      :data="chartData"
      :categories="chartCategories"
      :height="containerHeight"
      curve="monotone"
      index="round"
      x-label="Round"
      y-label="Points"
      points
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

      for (const player of props.players) {
        const points = player.points[round]

        if (points !== undefined) {
          dataPoint[String(player.id)] = points
        }
      }

      dataPoints.push(dataPoint)
    }

    return dataPoints
  })

  const chartCategories = computed(() => {
    return props.players.map(p => ({
      key: String(p.id),
      label: p.name,
    }))
  })
</script>

<style scoped>

</style>
