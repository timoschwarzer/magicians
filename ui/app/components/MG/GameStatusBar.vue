<template>
  <div class="flex divide-x divide-solid divide-accented border-t border-accented overflow-x-auto text-nowrap">
    <div v-if="game !== null && game.state.type !== 'Setup'" class="p-2">
      Round <NumberFlow :value="game.state.round" />
    </div>

    <div v-if="player !== null" class="p-2">
      <MGPlayerView :player="player" />
    </div>

    <div v-if="canChangeTrumpColor" class="flex">
      <USelect
        :model-value="game?.trumpColor ?? null"
        @update:modelValue="(color: MG.CardColor | null) => emit('changeTrumpColor', color as MG.CardColor | null)"
        :items="trumpItems"
        class="rounded-none"
        variant="none"
        :color="trumpUiColor"
        :ui="{ content: 'min-w-fit' }"
        value-key="value"
      >
        <template #leading>
          <UIcon v-if="game?.trumpColor !== null" name="i-lucide-star" class="bg-white"/>
          <UIcon v-else name="i-lucide-star-off" class="bg-white"/>
        </template>
        <template #trailing>
          <UIcon name="i-lucide-chevron-up" class="bg-white"/>
        </template>
      </USelect>
    </div>

    <div v-if="canLowerPrediction || canRaisePrediction" class="flex">
      <UDropdownMenu :items="changePredictionMenuItems">
        <UButton icon="i-lucide-arrow-up-down" variant="link">Prediction</UButton>
      </UDropdownMenu>
    </div>

    <div v-if="canSignalReady && player !== null" class="flex">
      <UButton
        variant="link"
        :icon="player.readyToContinue ? 'i-lucide-check' : 'i-lucide-square'"
        :color="player.readyToContinue ? 'success' : undefined"
        @click="emit('signalReady', !player.readyToContinue)"
      >{{ signalReadyLabel }}
      </UButton>
    </div>

    <div v-if="game !== null && readyPlayerCount > 0" class="py-2 px-3 flex gap-2 items-center">
      <div class="flex gap-1 items-center">
        <UIcon name="i-lucide-circle-check-big" class="bg-green-500"/>
        <div><NumberFlow :value="readyPlayerCount" />/{{ game.players.length }}</div>
      </div>
    </div>

    <div class="grow"></div>

    <div v-if="fullscreenButtonVisible" class="flex">
      <UButton
        icon="i-lucide-fullscreen"
        variant="link"
        @click="enterFullscreen()"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"
  import type {SelectItem} from "@nuxt/ui/components/Select.vue"
  import NumberFlow from "@number-flow/vue"
  import type {DropdownMenuItem} from "@nuxt/ui/components/DropdownMenu.vue"
  import {useScreenOrientation} from "@vueuse/core"

  const props = defineProps<{
    game: MG.Game | null,
    player: MG.Player | null,
    signalReadyLabel: string,
    canSignalReady: boolean,
    canChangeTrumpColor: boolean,
  }>()

  const emit = defineEmits<{
    (on: "signalReady", ready: boolean): void,
    (on: "changeTrumpColor", color: MG.CardColor | null): void,
    (on: "changePrediction", prediction: number): void,
  }>()

  const screenOrientation = useScreenOrientation()

  const fullscreenButtonVisible = ref(false)
  const wakeLockSentinel = shallowRef<WakeLockSentinel | null>(null)
  const trumpItems = ref([
    {label: "Blue", value: "Blue", chip: {color: "blue"}},
    {label: "Green", value: "Green", chip: {color: "green"}},
    {label: "Red", value: "Red", chip: {color: "red"}},
    {label: "Yellow", value: "Yellow", chip: {color: "yellow"}},
    {label: "No Trump", value: null},
  ] satisfies SelectItem[])
  const canRaisePrediction = computed(() => {
    if (
      props.game === null ||
      props.player === null ||
      props.player.prediction === null ||
      props.game.state.type !== "Playing"
    ) {
      return false
    }

    return props.player.prediction < props.game.state.round
  })
  const canLowerPrediction = computed(() => {
    if (
      props.game === null ||
      props.player === null ||
      props.player.prediction === null ||
      props.game.state.type !== "Playing"
    ) {
      return false
    }

    return props.player.prediction > 0
  })
  const changePredictionMenuItems = computed<DropdownMenuItem[]>(() => [
    {label: "Raise my prediction", disabled: !canRaisePrediction.value, icon: "i-lucide-plus", onSelect: () => emit("changePrediction", (props.player?.prediction ?? 0) + 1)},
    {label: "Lower my prediction", disabled: !canLowerPrediction.value, icon: "i-lucide-minus", onSelect: () => emit("changePrediction", (props.player?.prediction ?? 0) - 1)},
  ])

  const readyPlayerCount = computed(() => {
    if (props.game === null) {
      return 0
    }

    return props.game.players.filter(p => p.readyToContinue).length
  })

  const trumpUiColor = computed(() => {
    switch (props.game?.trumpColor) {
      case "Blue":
        return "blue"
      case "Green":
        return "green"
      case "Red":
        return "red"
      case "Yellow":
        return "yellow"
      default:
        return undefined
    }
  })

  function updateFullscreenButtonVisibility() {
    const fullScreenEnabled = !document.fullscreenEnabled || document.fullscreenElement !== null
    fullscreenButtonVisible.value = !fullScreenEnabled

    if (screenOrientation.isSupported.value) {
      try {
        screenOrientation.unlockOrientation()
      } catch (e) {
        console.error("Failed to unlock screen orientation: ", e)
      }
    }
  }

  onMounted(() => {
    document.addEventListener("fullscreenchange", updateFullscreenButtonVisibility)
    updateFullscreenButtonVisibility()
  })

  onBeforeUnmount(async () => {
    document.removeEventListener("fullscreenchange", updateFullscreenButtonVisibility)

    if (wakeLockSentinel.value !== null) {
      await wakeLockSentinel.value.release()
    }
  })

  async function enterFullscreen() {
    await document.body.requestFullscreen({
      navigationUI: "hide",
    })

    try {
      wakeLockSentinel.value = await navigator.wakeLock.request("screen")
    } catch (e) {
      console.error("Failed to acquire wakelock: ", e)
    }

    if (screenOrientation.isSupported.value) {
      try {
        await screenOrientation.lockOrientation("landscape")
      } catch (e) {
        console.error("Failed to lock screen orientation: ", e)
      }
    } else {
      console.warn("Screen orientation API is not supported")
    }

    // Workaround for the issue that the viewport size changes instantly after
    // going full screen in Chrome, which hides the bottom toolbar *sometimes*.
    setTimeout(() => {
      window.scrollTo({top: 0, left: 0, behavior: "instant"})
    }, 100)
  }
</script>

<style scoped>

</style>