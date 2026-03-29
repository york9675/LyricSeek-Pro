<template>
  <BaseModal
    :title="t('config.title')"
    @beforeOpen="beforeOpenHandler"
    @close="emit('close')"
    body-class="flex flex-col h-full justify-between overflow-y-auto"
  >
    <div class="flex flex-col gap-8">
      <div>
        <label class="group-label mb-4">{{ t('config.groups.common') }}</label>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">{{ t('config.downloadLyricsFor.label') }}</label>

          <RadioButton
              class="mb-1"
              v-model="downloadLyricsFor"
              name="download-lyrics-for"
              id="download-lyrics-for-all"
              value="all"
            >
              {{ t('config.downloadLyricsFor.allSongs') }}
            </RadioButton>

            <RadioButton
              class="mb-1"
              v-model="downloadLyricsFor"
              name="download-lyrics-for"
              id="skip-synced"
              value="skipSynced"
            >
              {{ t('config.downloadLyricsFor.noSynced') }}
            </RadioButton>

            <RadioButton
              class="mb-1"
              v-model="downloadLyricsFor"
              name="download-lyrics-for"
              id="skip-plain"
              value="skipPlain"
            >
              {{ t('config.downloadLyricsFor.noPlainOrSynced') }}
            </RadioButton>
        </div>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">{{ t('config.searchSettings.label') }}</label>

          <CheckboxButton
              v-model="showLineCount"
              name="show-line-count"
              id="show-line-count"
            >
              {{ t('config.searchSettings.showLineCount') }}
          </CheckboxButton>

          <CheckboxButton
              class="mt-2"
              v-model="preferSyncedLyrics"
              name="prefer-synced-lyrics"
              id="prefer-synced-lyrics"
            >
              {{ t('config.searchSettings.preferSynced') }}
          </CheckboxButton>
        </div>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">{{ t('config.language.label') }}</label>

          <select
            id="language"
            v-model="editingLocale"
            class="input w-full py-1.5 px-2"
          >
            <option value="system">{{ t('config.language.system') }}</option>
            <option value="en">{{ t('config.language.en') }}</option>
            <option value="zh-TW">{{ t('config.language.zhTw') }}</option>
          </select>
        </div>

        <div class="flex flex-col mb-4">
          <label class="block mb-2 child-label">{{ t('config.themeMode.label') }}</label>

          <div class="flex gap-4">
            <RadioButton
              v-model="editingThemeMode"
              name="theme-mode"
              id="theme-auto"
              value="auto"
            >
              {{ t('config.themeMode.auto') }}
            </RadioButton>

            <RadioButton
              v-model="editingThemeMode"
              name="theme-mode"
              id="theme-light"
              value="light"
            >
              {{ t('config.themeMode.light') }}
            </RadioButton>

            <RadioButton
              v-model="editingThemeMode"
              name="theme-mode"
              id="theme-dark"
              value="dark"
            >
              {{ t('config.themeMode.dark') }}
            </RadioButton>
          </div>
        </div>

        <div class="flex flex-col">
          <label class="block mb-2 child-label" for="lrclib-instance">{{ t('config.lrclibInstance') }}</label>
          <input id="lrclib-instance" type="text" v-model="editingLrclibInstance" placeholder="https://" class="input px-4 h-8">
        </div>

        <div class="flex flex-col mt-4">
          <label class="block mb-2 child-label">{{ t('config.providersOrder.label') }}</label>
          <div class="text-xs text-brave-35 dark:text-brave-70 mb-2">
            {{ t('config.providersOrder.description') }}
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
                  <span>{{ editingEnabledProviders.includes(provider) ? t('common.enabled') : t('common.disabled') }}</span>
                </label>

                <button
                  type="button"
                  class="button rounded-full px-3 py-1 text-[0.65rem]"
                  :class="index === 0 ? 'button-disabled' : 'button-normal'"
                  :disabled="index === 0"
                  :aria-label="t('config.providersOrder.moveUp')"
                  @click="moveProvider(index, -1)"
                >
                  <span class="text-base leading-none">↑</span>
                </button>
                <button
                  type="button"
                  class="button rounded-full px-3 py-1 text-[0.65rem]"
                  :class="index === editingProvidersOrder.length - 1 ? 'button-disabled' : 'button-normal'"
                  :disabled="index === editingProvidersOrder.length - 1"
                  :aria-label="t('config.providersOrder.moveDown')"
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
        <label class="group-label mb-4">{{ t('config.groups.externalLinks') }}</label>

        <div class="flex items-start">
          <CheckboxButton
            v-model="lastfmLinksEnabled"
            name="lastfm-links"
            id="lastfm-links"
          >
            {{ t('config.externalLinks.lastfm') }}
          </CheckboxButton>
        </div>
      </div>

      <div>
        <label class="group-label mb-4">{{ t('config.groups.experimental') }}</label>

        <div class="flex items-start">
          <CheckboxButton
            v-model="tryEmbedLyrics"
            name="try-embed-lyrics"
            id="try-embed-lyrics"
          >
            <div class="flex flex-col">
              <span class="mb-0.5">{{ t('config.experimental.embedLyrics') }}</span>
              <span class="text-xs text-yellow-700 dark:text-yellow-400">{{ t('config.experimental.warning') }}</span>
            </div>
          </CheckboxButton>
        </div>
      </div>

      <div class="flex flex-col gap-1">
        <a href="#" class="link" @click="refreshLibrary">{{ t('config.actions.refreshLibrary') }}</a>
        <a href="#" class="link" @click="uninitializeLibrary">{{ t('config.actions.manageDirectories') }}</a>
      </div>
    </div>

    <template #footer>
      <button class="button button-primary px-8 py-2 rounded-full" @click="save">{{ t('common.save') }}</button>
    </template>
  </BaseModal>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useGlobalState } from '../../composables/global-state'
import RadioButton from '@/components/common/RadioButton.vue'
import CheckboxButton from '@/components/common/CheckboxButton.vue'
import { getLocalePreference, setLocale } from '@/i18n'

const { setThemeMode, setLrclibInstance } = useGlobalState()
const { t } = useI18n()

const emit = defineEmits(['close', 'refreshLibrary', 'uninitializeLibrary'])

const downloadLyricsFor = ref('skipPlain')
const skipTracksWithSyncedLyrics = ref(true)
const skipTracksWithPlainLyrics = ref(true)
const showLineCount = ref(true)
const tryEmbedLyrics = ref(true)
const preferSyncedLyrics = ref(true)
const lastfmLinksEnabled = ref(true)
const editingThemeMode = ref('light')
const editingLocale = ref('system')
const editingLrclibInstance = ref('')
const defaultProvidersOrder = ['lrclib', 'netease', 'simpmusic', 'genius']
const editingProvidersOrder = ref([...defaultProvidersOrder])
const editingEnabledProviders = ref([...defaultProvidersOrder])

const providerLabels = computed(() => ({
  lrclib: t('config.providers.lrclib.label'),
  netease: t('config.providers.netease.label'),
  simpmusic: t('config.providers.simpmusic.label'),
  genius: t('config.providers.genius.label')
}))

const providerDescriptions = computed(() => ({
  lrclib: t('config.providers.lrclib.description'),
  netease: t('config.providers.netease.description'),
  simpmusic: t('config.providers.simpmusic.description'),
  genius: t('config.providers.genius.description')
}))

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
    tryEmbedLyrics: tryEmbedLyrics.value,
    preferSyncedLyrics: preferSyncedLyrics.value,
    themeMode: editingThemeMode.value,
    lrclibInstance: editingLrclibInstance.value,
    providersOrder: editingProvidersOrder.value,
    enabledProviders: editingEnabledProviders.value,
    lastfmLinksEnabled: lastfmLinksEnabled.value
  })
  setThemeMode(editingThemeMode.value)
  setLrclibInstance(editingLrclibInstance.value)
  setLocale(editingLocale.value)
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
  editingLocale.value = getLocalePreference()
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
  tryEmbedLyrics.value = config.try_embed_lyrics
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
</script>
