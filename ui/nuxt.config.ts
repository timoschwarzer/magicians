// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: {enabled: true},
  modules: ['@nuxt/ui', '@nuxt/eslint'],
  css: ['~/assets/style/main.css'],
  ssr: false,
  nitro: {
    experimental: {
      websocket: true,
    },
  },
  ui: {
    theme: {
      colors: ['primary', 'secondary', 'info', 'success', 'warning', 'error', 'neutral', 'green', 'blue', 'red', 'yellow']
    },
  },
  app: {
    head: {
      title: "Magicians",
      titleTemplate: "%s - Magicians",
      htmlAttrs: {
        lang: "en",
      },
      // link: [
      //   {rel: "icon", type: "image/png", href: "/favicon-96x96.png", sizes: "96x96"},
      //   {rel: "icon", type: "image/svg+xml", href: "/favicon.svg"},
      //   {rel: "shortcut icon", href: "/favicon.ico"},
      //   {rel: "apple-touch-icon", sizes: "180x180", href: "/apple-touch-icon.png"},
      //   {rel: "manifest", href: "/site.webmanifest"},
      // ],
    },
  },
})
