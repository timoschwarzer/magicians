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
          <div v-if="game.trumpColor !== null" :key="trumpUiColor" class="-z-10 absolute inset-0 pointer-events-none">
            <div :class="`from-${trumpUiColor}-600`" class="bg-linear-to-b absolute left-0 top-0 bottom-[33%] right-0"></div>
          </div>
        </transition>
        <transition name="fade">
          <div
            v-if="showBeginningPlayerMessage"
            class="absolute z-10 inset-0 pointer-events-none flex items-center justify-center bg-white text-black text-2xl"
          >
            You start this round.
          </div>
        </transition>

        <transition name="slide-y" mode="out-in">
          <div v-if="game.state.type === 'Setup'" key="Setup" class="p-2 h-full flex flex-col justify-center items-center gap-3">
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
              <MGReadyButton :disabled="game.players.length < 2" :ready="selfIsReady" @signal-ready="signalReady" />
            </div>
          </div>
          <div v-else-if="game.state.type === 'Predicting' || game.state.type === 'Playing'" class="h-full grid grid-cols-[1fr_2fr] gap-1">
            <div class="flex justify-center items-center col-1 px-4">
              <MGPredictions
                :game="game"
                :highlight-index="game.state.type === 'Predicting' ? game.state.playerIndex : undefined"
              />
            </div>
            <transition name="slide-y" mode="out-in">
              <div
                v-if="game.state.type === 'Playing' || (game.state.type === 'Predicting' && player.prediction !== null)"
                class="p-2 h-full flex flex-col justify-center items-center gap-3 col-2 overflow-hidden"
              >
                <transition name="fade" mode="out-in">
                  <div
                    :key="player.prediction ?? -1"
                    :class="{'rotate-180': game.state.type === 'Playing'}"
                    class="transition-transform duration-500 flex justify-center items-center text-screen grow decoration-2 underline underline-offset-16"
                  >
                    {{ player.prediction }}
                  </div>
                </transition>
              </div>
              <div v-else key="Predicting" class="h-full col-2">
                <transition name="slide-y" mode="out-in">
                  <div v-if="game.state.playerIndex === playerIndex" class="p-2 mx-4 h-full flex flex-col justify-center items-center gap-3 text-center">
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
                  </div>
                  <div v-else class="p-2 h-full flex justify-center items-center">
                    Waiting for {{ game.players[game.state.playerIndex]?.name ?? '???' }}...
                  </div>
                </transition>
              </div>
            </transition>
          </div>
          <div v-else-if="game.state.type === 'DistributingPoints'" key="DistributingPoints" class="h-full">
            <transition name="slide-y" mode="out-in">
              <div v-if="player.readyToContinue" class="p-2 h-full flex justify-center items-center">
                Waiting for other players...
              </div>
              <div v-else class="p-2 h-full flex flex-col justify-center items-center gap-3">
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
              </div>
            </transition>
          </div>
          <div v-else-if="game.state.type === 'Idle'" key="Idle" class="grid h-full auto-cols-fr">
            <div class="p-2 relative flex flex-col justify-center items-center gap-3 overflow-y-auto">
              <h2 class="text-3xl">Round {{ game.state.round }}</h2>
              <MGLeaderboard :game="game" :highlight-index="playerIndex" :give-cards-index="game.state.lastPlayerIndex" />
            </div>
            <div v-if="playerIndex === game.state.lastPlayerIndex" class="col-2 p-2 relative flex flex-col justify-center items-center gap-3 overflow-y-auto">
              <div>
                You have to deal <strong>{{ game.state.round }} card<template v-if="game.state.round !== 1">s</template></strong>.
              </div>

              <UButton @click="cardsDealtDialogOpen = true">Continue</UButton>

              <UModal v-model:open="cardsDealtDialogOpen" title="Select Trump Color">
                <template #body>
                  <div class="gap-2 grid grid-flow-col auto-cols-fr">
                    <UButton @click="setTrumpColorAndContinue('Blue')" color="blue">Blue</UButton>
                    <UButton @click="setTrumpColorAndContinue('Green')" color="green">Green</UButton>
                    <UButton @click="setTrumpColorAndContinue('Red')" color="red">Red</UButton>
                    <UButton @click="setTrumpColorAndContinue('Yellow')" color="yellow">Yellow</UButton>
                    <UButton @click="setTrumpColorAndContinue(null)" variant="subtle">None</UButton>
                  </div>
                </template>
              </UModal>
            </div>
          </div>
        </transition>
      </template>
      <template v-else>
        <div class="h-full flex flex-col gap-2 justify-center items-center p-2">
          You cannot join this game because it is already in progress.

          <MGLeaderboard
            v-if="game !== null"
            :game="game"
          />
        </div>
      </template>
    </div>
    <MGGameStatusBar
      :game="game"
      :player="player"
      :can-signal-ready="playerIndex !== null && game?.state?.type === 'Playing'"
      :can-change-trump-color="playerIndex !== null && (game?.state?.type === 'Idle' || game?.state?.type === 'Predicting' || game?.state?.type === 'Playing')"
      :signal-ready-label="game?.state?.type === 'Idle' ? 'I\'m ready to play' : 'Round Finished'"
      @change-prediction="changePrediction"
      @signal-ready="signalReady"
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
  const nameInput = ref("")
  const cardsDealtDialogOpen = ref(false)
  const showBeginningPlayerMessage = ref(false)
  const canShare = ref(typeof window.navigator.share === "function")

  const player = computed(() => {
    if (game.value === null || playerIndex.value === null) {
      return null
    }

    return game.value.players[playerIndex.value] ?? null
  })
  const selfIsReady = computed(() => {
    return player.value?.readyToContinue ?? false
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
    return game.value !== null && game.value.state.type === "Setup" && playerIndex.value === null
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

  watch(() => game.value?.state?.type, (newV, oldV) => {
    console.log(newV, oldV)

    switch (game.value?.state?.type) {
      case "Playing":
        cardsDealtDialogOpen.value = false

        if (playerIndex.value === game.value.state.firstPlayerIndex) {
          setTimeout(() => {
            showBeginningPlayerMessage.value = true
            setTimeout(() => showBeginningPlayerMessage.value = false, 2000)
          }, 1000)
        }
        break
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

  function signalReady(ready: boolean) {
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

  function setTrumpColorAndContinue(to: MG.CardColor | null) {
    setTrumpColor(to)
    signalReady(true)
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