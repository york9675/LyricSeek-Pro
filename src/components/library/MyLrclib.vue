<template>
  <div class="flex flex-col justify-center gap-4 items-center w-full h-full p-4" v-show="props.isActive">
    <h1 class="text-2xl md:text-3xl font-bold tracking-tight text-brave-20 dark:text-brave-95">
      LyricSeek Pro
    </h1>

    <form
      class="flex items-center rounded-full w-full max-w-screen-sm h-auto overflow-hidden bg-brave-98 dark:bg-brave-5 transition"
      :class="{ 'ring ring-brave-30/30': inputActive }"
      @submit.prevent="onSubmit"
    >
      <input
        id="search-keyword"
        type="text"
        v-model="keyword"
        class="outline-none grow h-12 px-6 bg-brave-98 dark:bg-brave-5 placeholder:text-brave-30/30 text-brave-20 dark:text-brave-95 dark:placeholder:text-brave-70/30"
        :placeholder="t('myLrclib.searchPlaceholder')"
        @focus="inputActive = true"
        @blur="inputActive = false"
        autofocus
      >

      <button
        type="button"
        class="rounded-full button h-12 w-12 m-1 flex items-center justify-center"
        :class="showAdvanced ? 'button-primary text-white' : 'button-normal'"
        :title="t('myLrclib.toggleAdvanced')"
        @click="showAdvanced = !showAdvanced"
      >
        <Cog class="text-2xl" />
      </button>

      <button type="submit" class="rounded-full button button-normal h-12 w-12 m-1 flex items-center justify-center" :title="t('myLrclib.search')">
        <Magnify class="text-2xl" />
      </button>
    </form>

    <Transition name="pop-fade">
      <div v-if="showAdvanced" class="w-full max-w-screen-sm rounded-lg bg-brave-98 dark:bg-brave-5 p-4 border border-brave-90 dark:border-brave-30">
        <div class="grid grid-cols-2 gap-2">
          <div>
            <label for="search-artist" class="group-label mb-1">{{ t('myLrclib.artistOptional') }}</label>
            <input
              id="search-artist"
              type="text"
              v-model="artistName"
              class="input w-full py-1.5 px-2"
              :placeholder="t('myLrclib.artist')"
            >
          </div>

          <div>
            <label for="search-album" class="group-label mb-1">{{ t('myLrclib.albumOptional') }}</label>
            <input
              id="search-album"
              type="text"
              v-model="albumName"
              class="input w-full py-1.5 px-2"
              :placeholder="t('myLrclib.album')"
            >
          </div>

          <div>
            <label for="search-provider" class="group-label mb-1">{{ t('myLrclib.provider') }}</label>
            <select id="search-provider" v-model="selectedProvider" class="input w-full py-1.5 px-2">
              <option value="auto">{{ t('myLrclib.providerAuto') }}</option>
              <option
                v-for="provider in providerOptions"
                :key="provider"
                :value="provider"
              >
                {{ providerLabels[provider] || provider }}
              </option>
            </select>
          </div>

          <div v-if="selectedProvider === 'lrclib'">
            <label for="search-lrclib-tag" class="group-label mb-1">{{ t('myLrclib.lrclibTagOptional') }}</label>
            <input
              id="search-lrclib-tag"
              type="text"
              v-model="lrclibTag"
              class="input w-full py-1.5 px-2"
              :placeholder="t('myLrclib.tag')"
            >
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="slide-fade">
      <SearchResult
        v-if="searchingParams"
        :search-params="searchingParams"
        @back="searchingParams = null"
      />
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Magnify, Cog } from 'mdue'
import SearchResult from './my-lrclib/SearchResult.vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  isActive: {
    type: Boolean,
    default: false,
  },
})

const searchingParams = ref(null)
const keyword = ref('')
const artistName = ref('')
const albumName = ref('')
const selectedProvider = ref('auto')
const lrclibTag = ref('')
const providerOptions = ref([])
const showAdvanced = ref(false)
const inputActive = ref(false)
const { t } = useI18n()

const providerLabels = {
  lrclib: 'LRCLIB',
  netease: 'NetEase',
  simpmusic: 'SimpMusic',
  genius: 'Genius'
}

const loadConfig = async () => {
  const config = await invoke('get_config')
  providerOptions.value = (config.providers_order || []).filter((provider) =>
    (config.enabled_providers || []).includes(provider)
  )
}

const onSubmit = () => {
  searchingParams.value = {
    keyword: keyword.value,
    artistName: artistName.value,
    albumName: albumName.value,
    provider: selectedProvider.value,
    lrclibTag: lrclibTag.value,
  }
}

onMounted(async () => {
  await loadConfig()
})

watch(() => props.isActive, async (isActive) => {
  if (isActive) {
    await loadConfig()
  }
})
</script>
