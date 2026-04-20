<template>
  <div class="flex flex-col h-screen">
    <div class="flex flex-col items-center justify-center grow text-center p-3">
      <h1 class="text-3xl md:text-4xl lg:text-5xl font-light mb-6">🧙 Magicians</h1>

      <div class="mb-8">
        Unofficial companion app for the card game Wizard.
      </div>

      <UButton
        size="xl"
        @click="createMap"
        icon="i-lucide-plus"
      >
        Create Game
      </UButton>
    </div>

    <footer class="text-center p-3 text-sm opacity-30">
      Made by <a href="https://timo.schwarzer.dev/">Timo</a>
    </footer>
  </div>
</template>

<script setup lang="ts">
  import type {MG} from "~/assets/script/mg"

  const axios = useAxios()
  const router = useRouter()

  useHead({
    titleTemplate: 'Magicians',
  })

  async function createMap() {
    const {data}: { data: MG.CreateGameResponse } = await axios.post("/game")

    await router.push({
      name: "game-id",
      params: {
        id: data.gameId,
      },
    })
  }
</script>

<style scoped>

</style>