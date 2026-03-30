<template>
  <BaseModal
    title="Configuration"
    @beforeOpen="beforeOpenHandler"
    @close="emit('close')"
    body-class="flex flex-col h-full justify-between overflow-y-auto"
  >
    <div class="flex flex-col gap-8">
      <div>
        <label class="group-label mb-4">Common</label>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">Download lyrics for</label>

          <RadioButton
              class="mb-1"
              v-model="downloadLyricsFor"
              name="download-lyrics-for"
              id="download-lyrics-for-all"
              value="all"
            >
              Download lyrics for all songs
            </RadioButton>

            <RadioButton
              class="mb-1"
              v-model="downloadLyricsFor"
              name="download-lyrics-for"
              id="skip-synced"
              value="skipSynced"
            >
              Download lyrics for songs without synced lyrics
            </RadioButton>

            <RadioButton
              class="mb-1"
              v-model="downloadLyricsFor"
              name="download-lyrics-for"
              id="skip-plain"
              value="skipPlain"
            >
              Download lyrics for songs without plain or synced lyrics
            </RadioButton>
        </div>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">Search settings</label>

          <CheckboxButton
              v-model="showLineCount"
              name="show-line-count"
              id="show-line-count"
            >
              Show the number of lines a lyric file has in the search menu
          </CheckboxButton>

          <CheckboxButton
              class="mt-2"
              v-model="preferSyncedLyrics"
              name="prefer-synced-lyrics"
              id="prefer-synced-lyrics"
            >
              Prefer synced lyrics first across providers (fallback to plain lyrics if needed)
          </CheckboxButton>

          <CheckboxButton
              class="mt-2"
              v-model="showCoverArtInTrackList"
              name="show-cover-art-in-track-list"
              id="show-cover-art-in-track-list"
            >
              Show cover art in track list
          </CheckboxButton>
        </div>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">Theme mode</label>

          <div class="flex gap-4">
            <RadioButton
              v-model="editingThemeMode"
              name="theme-mode"
              id="theme-auto"
              value="auto"
            >
              Auto
            </RadioButton>

            <RadioButton
              v-model="editingThemeMode"
              name="theme-mode"
              id="theme-light"
              value="light"
            >
              Light
            </RadioButton>

            <RadioButton
              v-model="editingThemeMode"
              name="theme-mode"
              id="theme-dark"
              value="dark"
            >
              Dark
            </RadioButton>
          </div>
        </div>

        <div class="flex flex-col">
          <label class="block mb-2 child-label" for="lrclib-instance">LRCLIB instance</label>
          <input id="lrclib-instance" type="text" v-model="editingLrclibInstance" placeholder="https://" class="input px-4 h-8">
        </div>

        <div class="flex flex-col mt-4">
          <label class="block mb-2 child-label">Lyrics provider order</label>
          <div class="text-xs text-brave-35 dark:text-brave-70 mb-2">
            Providers are tried from top to bottom when downloading lyrics.
          </div>

          <div class="flex flex-col gap-2">
            <div
              v-for="(provider, index) in editingProvidersOrder"
              :key="provider"
              class="flex items-center justify-between rounded-lg px-3 py-2 border border-brave-90 dark:border-brave-30 bg-brave-98 dark:bg-brave-10"
            >
              <div class="flex flex-col">
                <span class="text-sm font-bold">{{ providerLabels[provider] || provider }}</span>
                <span class="text-xs text-brave-35 dark:text-brave-70">{{ providerDescriptions[provider] || '' }}</span>
              </div>

              <div class="flex items-center gap-1">
                <label class="flex items-center gap-1 px-2 text-xs child-label cursor-pointer">
                  <input
                    type="checkbox"
                    class="accent-brave-30"
                    :checked="editingEnabledProviders.includes(provider)"
                    @change="toggleProviderEnabled(provider, $event.target.checked)"
                  >
                  <span>{{ editingEnabledProviders.includes(provider) ? 'Enabled' : 'Disabled' }}</span>
                </label>

                <button
                  type="button"
                  class="button rounded-full px-3 py-1 text-[0.65rem]"
                  :class="index === 0 ? 'button-disabled' : 'button-normal'"
                  :disabled="index === 0"
                  aria-label="Move provider up"
                  @click="moveProvider(index, -1)"
                >
                  <span class="text-base leading-none">↑</span>
                </button>
                <button
                  type="button"
                  class="button rounded-full px-3 py-1 text-[0.65rem]"
                  :class="index === editingProvidersOrder.length - 1 ? 'button-disabled' : 'button-normal'"
                  :disabled="index === editingProvidersOrder.length - 1"
                  aria-label="Move provider down"
                  @click="moveProvider(index, 1)"
                >
                  <span class="text-base leading-none">↓</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div>
        <label class="group-label mb-4">External Links</label>

        <div class="flex items-start">
          <CheckboxButton
            v-model="lastfmLinksEnabled"
            name="lastfm-links"
            id="lastfm-links"
          >
            Enable Last.fm links on player bar (title, artist, album)
          </CheckboxButton>
        </div>
      </div>

      <div>
        <label class="group-label mb-4">Experimental</label>

        <div class="flex items-start">
          <CheckboxButton
            v-model="tryEmbedLyrics"
            name="try-embed-lyrics"
            id="try-embed-lyrics"
          >
            <div class="flex flex-col">
              <span class="mb-0.5">Try to embed the lyrics to the track files when possible</span>
              <span class="text-xs text-yellow-700 dark:text-yellow-400">This option could corrupt your track files. Make sure to backup your library before enabling it.</span>
            </div>
          </CheckboxButton>
        </div>

        <div class="flex items-start mt-2 ml-6">
          <CheckboxButton
            v-model="embedLyricsOnly"
            name="embed-lyrics-only"
            id="embed-lyrics-only"
            :disabled="!tryEmbedLyrics"
          >
            Embed lyrics to track file only (do not create .txt/.lrc sidecar files)
          </CheckboxButton>
        </div>
      </div>

      <div class="flex flex-col gap-1">
        <a href="#" class="link" @click="refreshLibrary">Refresh my library for new changes...</a>
        <a href="#" class="link" @click="uninitializeLibrary">Add and remove scanning directories...</a>
      </div>
    </div>

    <template #footer>
      <button class="button button-primary px-8 py-2 rounded-full" @click="save">Save</button>
    </template>
  </BaseModal>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, watch } from 'vue'
import { useGlobalState } from '../../composables/global-state'
import RadioButton from '@/components/common/RadioButton.vue'
import CheckboxButton from '@/components/common/CheckboxButton.vue'

const { setThemeMode, setLrclibInstance, setShowCoverArtInTrackList } = useGlobalState()

const emit = defineEmits(['close', 'refreshLibrary', 'uninitializeLibrary'])

const downloadLyricsFor = ref('skipPlain')
const skipTracksWithSyncedLyrics = ref(true)
const skipTracksWithPlainLyrics = ref(true)
const showLineCount = ref(true)
const showCoverArtInTrackList = ref(true)
const tryEmbedLyrics = ref(true)
const embedLyricsOnly = ref(false)
const preferSyncedLyrics = ref(true)
const lastfmLinksEnabled = ref(true)
const editingThemeMode = ref('light')
const editingLrclibInstance = ref('')
const defaultProvidersOrder = ['lrclib', 'netease', 'simpmusic', 'genius']
const editingProvidersOrder = ref([...defaultProvidersOrder])
const editingEnabledProviders = ref([...defaultProvidersOrder])

const providerLabels = {
  lrclib: 'LRCLIB',
  netease: 'NetEase',
  simpmusic: 'SimpMusic',
  genius: 'Genius'
}

const providerDescriptions = {
  lrclib: 'Synced and plain lyrics from LRCLIB',
  netease: 'Lyrics from NetEase Cloud Music',
  simpmusic: 'Community lyrics from SimpMusic',
  genius: 'Plain lyrics from Genius'
}

const normalizeProvidersOrder = (providersOrder) => {
  const resolvedOrder = []

  if (Array.isArray(providersOrder)) {
    providersOrder.forEach((provider) => {
      const normalizedProvider = String(provider).trim().toLowerCase()
      if (defaultProvidersOrder.includes(normalizedProvider) && !resolvedOrder.includes(normalizedProvider)) {
        resolvedOrder.push(normalizedProvider)
      }
    })
  }

  defaultProvidersOrder.forEach((provider) => {
    if (!resolvedOrder.includes(provider)) {
      resolvedOrder.push(provider)
    }
  })

  return resolvedOrder
}

const normalizeEnabledProviders = (enabledProviders) => {
  const resolvedProviders = []

  if (Array.isArray(enabledProviders)) {
    enabledProviders.forEach((provider) => {
      const normalizedProvider = String(provider).trim().toLowerCase()
      if (defaultProvidersOrder.includes(normalizedProvider) && !resolvedProviders.includes(normalizedProvider)) {
        resolvedProviders.push(normalizedProvider)
      }
    })
  }

  if (resolvedProviders.length === 0) {
    return [...defaultProvidersOrder]
  }

  return resolvedProviders
}

const moveProvider = (index, direction) => {
  const nextIndex = index + direction
  if (nextIndex < 0 || nextIndex >= editingProvidersOrder.value.length) {
    return
  }

  const updated = [...editingProvidersOrder.value]
  const [movedProvider] = updated.splice(index, 1)
  updated.splice(nextIndex, 0, movedProvider)
  editingProvidersOrder.value = updated
}

const toggleProviderEnabled = (provider, enabled) => {
  if (enabled) {
    if (!editingEnabledProviders.value.includes(provider)) {
      editingEnabledProviders.value = [...editingEnabledProviders.value, provider]
    }
    return
  }

  if (editingEnabledProviders.value.length === 1) {
    return
  }

  editingEnabledProviders.value = editingEnabledProviders.value.filter((item) => item !== provider)
}

const save = async () => {
  await invoke('set_config', {
    skipTracksWithSyncedLyrics: skipTracksWithSyncedLyrics.value,
    skipTracksWithPlainLyrics: skipTracksWithPlainLyrics.value,
    showLineCount: showLineCount.value,
    showCoverArtInTrackList: showCoverArtInTrackList.value,
    tryEmbedLyrics: tryEmbedLyrics.value,
    embedLyricsOnly: embedLyricsOnly.value,
    preferSyncedLyrics: preferSyncedLyrics.value,
    themeMode: editingThemeMode.value,
    lrclibInstance: editingLrclibInstance.value,
    providersOrder: editingProvidersOrder.value,
    enabledProviders: editingEnabledProviders.value,
    lastfmLinksEnabled: lastfmLinksEnabled.value
  })
  setThemeMode(editingThemeMode.value)
  setLrclibInstance(editingLrclibInstance.value)
  setShowCoverArtInTrackList(showCoverArtInTrackList.value)
  emit('close')
}

const refreshLibrary = () => {
  emit('refreshLibrary')
  emit('close')
}

const uninitializeLibrary = () => {
  emit('uninitializeLibrary')
  emit('close')
}

const beforeOpenHandler = async () => {
  const config = await invoke('get_config')
  skipTracksWithSyncedLyrics.value = config.skip_tracks_with_synced_lyrics
  skipTracksWithPlainLyrics.value = config.skip_tracks_with_plain_lyrics

  if (skipTracksWithSyncedLyrics.value && !skipTracksWithPlainLyrics.value) {
    downloadLyricsFor.value = 'skipSynced'
  } else if (skipTracksWithPlainLyrics.value) {
    downloadLyricsFor.value = 'skipPlain'
  } else {
    downloadLyricsFor.value = 'all'
  }

  showLineCount.value = config.show_line_count
  showCoverArtInTrackList.value = config.show_cover_art_in_track_list
  tryEmbedLyrics.value = config.try_embed_lyrics
  embedLyricsOnly.value = config.embed_lyrics_only || false
  preferSyncedLyrics.value = config.prefer_synced_lyrics
  lastfmLinksEnabled.value = config.lastfm_links_enabled
  editingThemeMode.value = config.theme_mode
  editingLrclibInstance.value = config.lrclib_instance
  editingProvidersOrder.value = normalizeProvidersOrder(config.providers_order)
  editingEnabledProviders.value = normalizeEnabledProviders(config.enabled_providers)
}

watch(downloadLyricsFor, (newVal) => {
  if (newVal === 'skipSynced') {
    skipTracksWithSyncedLyrics.value = true
    skipTracksWithPlainLyrics.value = false
  } else if (newVal === 'skipPlain') {
    skipTracksWithSyncedLyrics.value = true
    skipTracksWithPlainLyrics.value = true
  } else {
    skipTracksWithSyncedLyrics.value = false
    skipTracksWithPlainLyrics.value = false
  }
})

watch(tryEmbedLyrics, (enabled) => {
  if (!enabled) {
    embedLyricsOnly.value = false
  }
})
</script>
