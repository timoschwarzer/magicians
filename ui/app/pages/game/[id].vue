<template>
  <div class="h-screen flex flex-col">
    <div class="grow shrink min-h-0 relative">
      <template v-if="canJoin">
        <div class="p-2 h-full flex flex-col justify-center items-center gap-1">
          <UInput v-model="nameInput" placeholder="Name"/>
          <UButton
            :disabled="nameInput.length <= 0"
            icon="i-lucide-user-round-plus"
            @click="join"
          >
            Join
          </UButton>

          <div class="mt-4">
            Join in clockwise order.
          </div>
        </div>
      </template>
      <template v-else-if="game !== null && playerIndex !== null && player !== null">
        <transition name="fade">
          <div v-if="game.trumpColor !== null" :key="trumpUiColor" class="absolute inset-0 pointer-events-none">
            <div :class="`from-${trumpUiColor}-600`" class="bg-linear-to-r absolute left-0 top-0 bottom-0  w-[40%]"></div>
            <div :class="`from-${trumpUiColor}-600`" class="bg-linear-to-l absolute right-0 top-0 bottom-0  w-[40%]"></div>
          </div>
        </transition>

        <template v-if="game.state.type === 'Setup'">
          <div class="p-2 h-full flex flex-col justify-center items-center gap-3">
            <div class="grid grid-cols-3 gap-6 text-2xl">
              <div v-if="nextPlayer !== null" class="flex items-center justify-end gap-1 pr-4 border-r opacity-75">
                <UIcon name="i-lucide-arrow-left" />
                {{ nextPlayer.name }}
              </div>
              <div v-else></div>
              <div class="text-center font-bold">
                {{ player.name }}
              </div>
              <div v-if="previousPlayer !== null" class="flex items-center justify-start gap-1 pl-4 border-l opacity-75">
                {{ previousPlayer.name }}
                <UIcon name="i-lucide-arrow-right" />
              </div>
              <div v-else></div>
            </div>

            <div class="flex gap-1">
              <UButton v-if="canShare" icon="i-lucide-share-2" size="xl" @click="shareOrCopyLink">Share Link</UButton>
              <UButton v-else icon="i-lucide-copy" size="xl" @click="shareOrCopyLink">Copy Link</UButton>
              <MGReadyButton :disabled="game.players.length < 2" :ready="selfIsReady" @signal-ready="onSignalReady" />
            </div>
          </div>
        </template>
        <template v-if="game.state.type === 'Predicting'">
          <div class="p-2 h-full flex flex-col justify-center items-center gap-3">
            <template v-if="player.readyToContinue">
              <div class="rotate-180 flex justify-center items-center text-screen grow">
                {{ player.prediction }}
              </div>
            </template>
            <template v-if="game.state.playerIndex === playerIndex">
              <span>Predict how many times you will win:</span>

              <div class="gap-2 text-2xl self-stretch flex justify-center flex-wrap">
                <UButton
                  v-for="i in possiblePredictions"
                  :key="i"
                  size="xl"
                  :disabled="game.state.disallowedPrediction === i"
                  class="w-12 px-4 text-2xl justify-center relative"
                  :color="i === predictionInput ? 'info' : undefined"
                  :variant="i === predictionInput ? 'solid' : 'subtle'"
                  @click="predictionInput = i"
                >
                  {{ i }}

                  <template v-if="game.state.disallowedPrediction === i">
                    <div class="bg-red-600 h-1 w-full absolute rotate-45"></div>
                    <div class="bg-red-600 h-1 w-full absolute -rotate-45"></div>
                  </template>
                </UButton>
              </div>

              <UButton :disabled="predictionInput === null" size="xl" icon="i-lucide-check" @click="submitPrediction">
                Submit
              </UButton>
            </template>
            <template v-else>
              Waiting for {{ game.players[game.state.playerIndex]?.name ?? '???' }}...
            </template>
          </div>
        </template>
        <template v-if="game.state.type === 'Playing'">
          <div class="h-full rotate-180 flex flex-col justify-center items-center grow">
            <h2 class="text-2xl">Prediction:</h2>
            <transition name="fade" mode="out-in">
              <div :key="player.prediction ?? -1" class="text-screen">
                {{ player.prediction }}
              </div>
            </transition>
          </div>
        </template>
        <template v-if="game.state.type === 'DistributingPoints'">
          <div class="p-2 h-full flex flex-col justify-center items-center gap-3">
            <template v-if="player.readyToContinue">
              Waiting for other players...
            </template>
            <template v-else>
              <span>How many times did you win?</span>

              <div class="gap-2 text-2xl self-stretch flex justify-center flex-wrap">
                <UButton
                  v-for="i in possibleWinCounts"
                  :key="i"
                  size="xl"
                  class="w-12 px-4 text-2xl justify-center underline-offset-4 decoration-zinc-700"
                  :class="{'underline': i === player.prediction}"
                  :variant="i === winCountInput ? 'solid' : 'subtle'"
                  @click="winCountInput = i"
                >
                  {{ i }}
                </UButton>
              </div>

              <UButton
                :disabled="winCountInput === null"
                :color="winCountInput === null ? undefined : (winCountInput === player.prediction ? 'green' : 'red')"
                size="xl"
                icon="i-lucide-check"
                @click="submitWinCount"
              >
                Submit
              </UButton>
            </template>
          </div>
        </template>
        <template v-if="game.state.type === 'Idle'">
          <div class="p-2 h-full relative flex flex-col justify-center items-center gap-3 overflow-y-auto">
            <h2 class="text-3xl">Round {{ game.state.round }}</h2>
            <MGLeaderboard :game="game" :highlight-index="playerIndex" :give-cards-index="game.state.lastPlayerIndex" />
          </div>
        </template>
      </template>
    </div>
    <MGGameStatusBar
      :game="game"
      :player="player"
      :can-signal-ready="game?.state?.type === 'Idle' || game?.state?.type === 'Playing'"
      :can-change-trump-color="game?.state?.type === 'Idle' || game?.state?.type === 'Predicting' || game?.state?.type === 'Playing'"
      :signal-ready-label="game?.state?.type === 'Idle' ? 'I\'m ready to play' : 'Finish Round'"
      @change-prediction="changePrediction"
      @signal-ready="onSignalReady"
      @change-trump-color="setTrumpColor"
    />
  </div>

  <UModal v-model:open="gameNotFoundDialogOpen" :close="false" :dismissible="false" title="Game not found">
    <template #body>
      This game does not exist (anymore). Please create a new one.

      <div class="flex justify-end mt-3">
        <UButton icon="i-lucide-home" :to="{name: 'index'}">Go to home page</UButton>
      </div>
    </template>
  </UModal>
</template>

<script lang="ts" setup>
  import type {MG} from "~/assets/script/mg"

  useHead({
    title: "Game",
  })

  const route = useRoute()
  const gameId = computed(() => route.params.id as string)
  const game = ref<MG.Game | null>(null)
  const playerIndex = ref<number | null>(null)
  const ws = shallowRef<WebSocket | null>(null)
  const gameNotFoundDialogOpen = ref(false)
  const wsReconnectTimeoutId = ref<number | null>(null)
  const predictionInput = ref<number | null>(null)
  const winCountInput = ref<number | null>(null)
  const nameInput = ref<string>("")

  const player = computed(() => {
    if (game.value === null || playerIndex.value === null) {
      return null
    }

    return game.value.players[playerIndex.value] ?? null
  })
  const selfIsReady = computed(() => {
    return player.value?.readyToContinue ?? false
  })
  const playerCount = computed(() => {
    return game.value?.players?.length ?? 0
  })
  const playersReady = computed(() => {
    return game.value?.players?.filter(p => p.readyToContinue)?.length ?? 0
  })
  const possiblePredictions = computed(() => {
    if (game.value === null || game.value.state.type !== "Predicting") {
      return []
    }

    const possiblePredictions = []
    for (let i = 0; i <= game.value.state.round; i++) {
      possiblePredictions.push(i)
    }
    return possiblePredictions
  })
  const possibleWinCounts = computed(() => {
    if (game.value === null || game.value.state.type !== "DistributingPoints") {
      return []
    }

    const possibleWinCounts = []
    for (let i = 0; i <= game.value.state.round; i++) {
      possibleWinCounts.push(i)
    }
    return possibleWinCounts
  })
  const canJoin = computed(() => {
    return game.value !== null && playerIndex.value === null
  })
  const previousPlayer = computed(() => {
    if (playerIndex.value === null || game.value === null || game.value.players.length < 2) {
      return null
    }

    return game.value.players.at(playerIndex.value - 1) ?? null
  })
  const nextPlayer = computed(() => {
    if (playerIndex.value === null || game.value === null || game.value.players.length < 2) {
      return null
    }

    return game.value.players.at((playerIndex.value + 1) % game.value.players.length) ?? null
  })
  const trumpUiColor = computed(() => {
    switch (game.value?.trumpColor) {
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
  const canShare = ref(typeof window.navigator.share === "function")

  watch(() => game.value?.state?.type, (newState) => {
    switch (newState) {
      case "Predicting":
        predictionInput.value = null
        break
      case "DistributingPoints":
        winCountInput.value = null
        break
    }
  })

  function closeWebSocketWithoutReconnect() {
    ws.value?.removeEventListener("close", onWebSocketClose)
    ws.value?.removeEventListener("error", onWebSocketError)
    ws.value?.close()
  }

  function reconnectWebsocketAfterDelay() {
    if (wsReconnectTimeoutId.value === null) {
      wsReconnectTimeoutId.value = setTimeout(() => {
        wsReconnectTimeoutId.value = null
        reconnectWebSocket()
      }, 2000)
    }
  }

  function onWebSocketError(_event: WebSocketEventMap["error"]) {
    closeWebSocketWithoutReconnect()
    reconnectWebsocketAfterDelay()
  }

  function onWebSocketClose(event: WebSocketEventMap["close"]) {
    if (event.code === 4404) {
      gameNotFoundDialogOpen.value = true;
      return;
    }

    closeWebSocketWithoutReconnect()
    reconnectWebsocketAfterDelay()
  }

  function sendCommand(command: MG.WebSocketCommand) {
    if (ws.value !== null) {
      ws.value.send(JSON.stringify(command))
    }
  }

  function reconnectWebSocket() {
    closeWebSocketWithoutReconnect()

    const baseURL = import.meta.dev
      ? "ws://localhost:8080/api"
      : `${window.location.protocol === "https:" ? "wss" : "ws"}://${window.location.host}/api`

    ws.value = new WebSocket(`${baseURL}/game/${gameId.value}/websocket`)

    ws.value.addEventListener("message", message => {
      const data: MG.WebSocketMessage = JSON.parse(message.data)

      switch (data.type) {
        case "CurrentGame":
          game.value = data.game
          break
        case "SetPlayerIndex":
          playerIndex.value = data.playerIndex
          break
        case "SetPlayerKey":
          window.localStorage.setItem(`player-key-${gameId.value}`, data.key)
          break
      }
    })

    ws.value.addEventListener("close", onWebSocketClose)
    ws.value.addEventListener("error", onWebSocketError)
    ws.value.addEventListener("open", () => {
      const playerKey = window.localStorage.getItem(`player-key-${gameId.value}`)

      if (playerKey !== null) {
        sendCommand({
          type: "Authenticate",
          key: playerKey,
        })
      }
    })
  }

  function onSignalReady(ready: boolean) {
    sendCommand({
      type: "SignalReady",
      ready,
    })
  }

  function submitPrediction() {
    sendCommand({
      type: "SubmitPrediction",
      prediction: predictionInput.value ?? 0,
    })
  }

  function submitWinCount() {
    sendCommand({
      type: "SubmitWinCount",
      winCount: winCountInput.value ?? 0,
    })
  }

  function changePrediction(to: number) {
    sendCommand({
      type: "ChangePrediction",
      prediction: to,
    })
  }

  function setTrumpColor(to: MG.CardColor | null) {
    sendCommand({
      type: "SetTrumpColor",
      trumpColor: to,
    })
  }

  function join() {
    sendCommand({
      type: "Join",
      name: nameInput.value,
    })
  }

  async function shareOrCopyLink() {
    if (canShare.value) {
      try {
        await navigator.share({
          url: location.href,
        })
        return
      } catch (error) {
        console.error("Failed to share URL, now trying to copy: ", error)
        canShare.value = false
      }
    }

    try {
      await navigator.clipboard.writeText(location.href)
    } catch (error) {
      console.error("Failed to copy: ", error)
    }
  }

  watch(gameId, async () => {
    if (import.meta.client) {
      reconnectWebSocket()
    }
  }, {immediate: true})

  onUnmounted(() => {
    ws.value?.close()
  })
</script>


<style>

</style>


<style scoped>
    .text-screen {
        line-height: 1;
        font-size: 50vh;
    }
</style>