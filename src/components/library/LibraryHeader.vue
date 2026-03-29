<template>
  <div class="px-4 py-2 h-12 flex justify-between gap-4 flex-none items-stretch">
    <div class="flex-1 ml-2">
      <MiniSearch v-if="props.activeTab === 'tracks'" />
    </div>

    <div class="flex-1 flex gap-4 justify-center items-center text-sm">
      <button
        class="tab"
        :class="{'active-tab': props.activeTab === 'tracks', 'inactive-tab': props.activeTab !== 'tracks'}"
        @click.prevent="$emit('changeActiveTab', 'tracks')"
      >
        {{ t('library.tabs.tracks') }}
      </button>
      <button
        class="tab"
        :class="{'active-tab': props.activeTab === 'albums', 'inactive-tab': props.activeTab !== 'albums'}"
        @click.prevent="$emit('changeActiveTab', 'albums')"
      >
        {{ t('library.tabs.albums') }}
      </button>
      <button
        class="tab"
        :class="{'active-tab': props.activeTab === 'artists', 'inactive-tab': props.activeTab !== 'artists'}"
        @click.prevent="$emit('changeActiveTab', 'artists')"
      >
        {{ t('library.tabs.artists') }}
      </button>

      <!-- Create a separator -->
      <div class="w-[2px] h-[70%] bg-brave-30/30" />

      <button
        class="tab"
        :class="{'active-tab': props.activeTab === 'my-lrclib', 'inactive-tab': props.activeTab !== 'my-lrclib'}"
        @click.prevent="$emit('changeActiveTab', 'my-lrclib')"
      >
        <span class="flex items-center gap-1">
          <Magnify class="text-sm" />
          {{ t('library.tabs.search') }}
        </span>
      </button>
    </div>

    <div class="flex-1 flex justify-end items-center gap-1">
      <button v-if="isBuildingQueue" class="button button-disabled px-4 py-1.5 h-full min-w-[12rem] text-xs rounded-full" @click.prevent="$emit('showDownloadViewer')" disabled>
        <div class="animate-spin text-sm"><Loading /></div>
        <div class="flex gap-1">
          <div>{{ t('library.header.preparing') }}</div>
        </div>
      </button>

      <button v-else-if="isDownloading && downloadedCount !== totalCount" class="button button-working h-full min-w-[12rem] px-4 py-1.5 text-xs rounded-full" @click.prevent="$emit('showDownloadViewer')">
        <div class="animate-spin text-sm"><Loading /></div>
        <div class="flex gap-1">
          <div>{{ t('library.header.downloading') }}</div>
          <div>{{ downloadedCount }}/{{ totalCount }}</div>
        </div>
      </button>

      <button v-else-if="isDownloading" class="button button-done h-full min-w-[12rem] px-4 py-1.5 text-xs rounded-full" @click.prevent="$emit('showDownloadViewer')">
        <div class="text-sm"><Check /></div>
        <span>
          {{ t('library.header.downloadedSummary', { downloaded: downloadedCount, total: totalCount }) }}
        </span>
      </button>

      <button v-else class="button button-primary px-4 py-1.5 h-full min-w-[12rem] text-xs rounded-full" @click.prevent="downloadAllLyrics">
        <div class="text-sm"><DownloadMultiple /></div>
        <span>
          {{ t('library.header.downloadAllLyrics') }}
        </span>
      </button>

      <VDropdown theme="lyricseekpro-dropdown" placement="top-end" class="h-full aspect-square">
        <button
          class="button button-normal h-full aspect-square rounded-full"
        >
          <DotsVertical />
        </button>
        <template #popper>
          <div class="dropdown-container">
            <button class="dropdown-item" @click="$emit('refreshLibrary')" v-close-popper>
              <Refresh class="text-brave-20 dark:text-brave-90" />
              <span class="text-brave-20 dark:text-brave-90 text-sm font-bold">{{ t('library.header.refreshLibrary') }}</span>
            </button>
            <button class="dropdown-item" @click="$emit('uninitializeLibrary')" v-close-popper>
              <FolderMultiple class="text-brave-20 dark:text-brave-90" />
              <span class="text-brave-20 dark:text-brave-90 text-sm font-bold">{{ t('library.header.manageDirectories') }}</span>
            </button>
            <button class="dropdown-item" @click="$emit('showConfig')" v-close-popper>
              <Cog class="text-brave-20 dark:text-brave-90" />
              <span class="text-brave-20 dark:text-brave-90 text-sm font-bold">{{ t('library.header.settings') }}</span>
            </button>
            <button class="dropdown-item" @click="$emit('showAbout')" v-close-popper>
              <Information class="text-brave-20 dark:text-brave-90" />
              <span class="text-brave-20 dark:text-brave-90 text-sm font-bold">{{ t('library.header.about') }}</span>
            </button>
          </div>
        </template>
      </VDropdown>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { DownloadMultiple, Loading, Check, Cog, Information, DotsVertical, Refresh, FolderMultiple, Magnify } from 'mdue'
import { useDownloader } from '@/composables/downloader.js'
import MiniSearch from './MiniSearch.vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps(['activeTab'])
defineEmits(['changeActiveTab', 'showConfig', 'showAbout', 'showDownloadViewer', 'refreshLibrary', 'uninitializeLibrary'])
const { t } = useI18n()

const { isDownloading, totalCount, downloadedCount, addToQueue } = useDownloader()

const isBuildingQueue = ref(false)

const downloadAllLyrics = async () => {
  isBuildingQueue.value = true

  try {
    const config = await invoke('get_config')
    let downloadTrackIds = await invoke('get_track_ids', {
      searchQuery: '',
      syncedLyricsTracks: !config.skip_tracks_with_synced_lyrics,
      plainLyricsTracks: !config.skip_tracks_with_plain_lyrics,
      instrumentalTracks: !config.skip_tracks_with_synced_lyrics && !config.skip_tracks_with_plain_lyrics, // Treat instrumental tracks as either synced or plain lyrics tracks
      noLyricsTracks: true,
    })
    addToQueue(downloadTrackIds)
  } catch (error) {
    // TODO handle error by showing an error popup, etc...
    console.error(error)
  } finally {
    isBuildingQueue.value = false
  }
}
</script>

<style scoped>
.active-tab {
  @apply text-brave-15 border-brave-15 dark:text-white dark:border-brave-30;
}

.inactive-tab {
  @apply text-brave-15/50 hover:text-brave-15/80 border-transparent dark:text-white/50 dark:hover:text-brave-95/80;
}

.tab {
  @apply transition font-extrabold border-b-2 outline-none py-1;
}

.dropdown-container {
  @apply p-1 min-w-[10rem];
}

.dropdown-item {
  @apply flex items-center px-2 py-1 hover:bg-brave-90 dark:hover:bg-brave-15 rounded cursor-pointer h-8 gap-1 w-full;
}
</style>
