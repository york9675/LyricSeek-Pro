<template>
  <div ref="parentRef" class="secondary-page">
    <div
      :style="{ height: `${totalSize}px`, width: '100%', position: 'relative' }"
    >
      <div class="mb-4">
        <button
          class="button button-normal transition rounded-full p-4"
          @click="$emit('back')"
        >
          <ArrowLeft />
        </button>
      </div>

      <!-- Album header: cover on left, info and actions on right -->
      <div class="flex items-start gap-4 sm:gap-6 mb-8">
        <div v-if="coverImageUrl" class="w-48 h-48 rounded-lg overflow-hidden shadow-xl flex-none">
          <img :src="coverImageUrl" :alt="album.name" class="w-full h-full object-cover">
        </div>
        <div v-else class="w-48 h-48 rounded-lg bg-brave-90 dark:bg-brave-20 flex items-center justify-center shadow-xl flex-none">
          <div class="text-6xl">💿</div>
        </div>

        <div class="h-48 flex flex-col justify-between min-w-0 flex-1">
          <div class="flex-1 flex items-center">
            <div class="flex flex-col gap-1 min-w-0">
              <div class="text-thin text-xl text-brave-10 dark:text-white line-clamp-2">
                {{ album.name }}
              </div>
              <div class="flex items-center gap-2">
                <div class="text-sm text-brave-30 group-hover:text-brave-20 transition dark:text-white">{{ album.tracks_count }} tracks</div>
                <div class="border-r border-brave-80 h-3 flex-none"></div>
                <div class="text-sm text-brave-30 group-hover:text-brave-20 transition dark:text-white line-clamp-1">{{ album.artist_name }}</div>
              </div>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <button class="button button-normal px-4 py-2 text-xs rounded-full" @click.prevent="downloadAlbumLyrics">
              <div class="text-sm"><DownloadMultiple /></div>
              <span>
                Download album lyrics
              </span>
            </button>
            <a
              v-if="lastfmLinksEnabled && lastfmAlbumUrl"
              :href="lastfmAlbumUrl"
              target="_blank"
              rel="noopener noreferrer"
              class="button button-normal p-2 rounded-full"
              title="Open album on Last.fm"
              aria-label="Open album on Last.fm"
            >
              <OpenInNew />
            </a>
          </div>
        </div>
      </div>

      <div class="w-full">
        <div class="w-full flex">
          <div class="text-xs text-brave-30/70 font-bold flex w-full dark:text-brave-95">
            <div class="text-right flex-none w-[5%] p-1 pr-2">#</div>
            <div class="text-left flex-none w-[60%] p-1">Track</div> <!-- Adjusted width percentage -->
            <div class="text-right flex-none w-[10%] p-1">Duration</div>
            <div class="text-center flex-none w-[10%] p-1">Lyrics</div>
            <div class="text-right flex-none w-[15%] p-1"></div>
          </div>
        </div>
        <div class="w-full flex flex-col">
          <div
            v-for="virtualRow in virtualRows"
            :key="virtualRow.index"
            class="group flex flex-col w-full"
            :style="{
              position: 'absolute',
              top: 0,
              left: 0,
              width: '100%',
              height: `${virtualRow.size}px`,
              transform: `translateY(${virtualRow.start}px)`,
              }"
          >
            <TrackItem
              :trackId="virtualRow.key"
              :isShowTrackNumber="true"
              :showCover="false"
              @play-track="playTrack"
              @download-lyrics="downloadLyrics"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ArrowLeft, DownloadMultiple, OpenInNew } from 'mdue'
import { useVirtualizer } from '@tanstack/vue-virtual'
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import TrackItem from '../track-list/TrackItem.vue'
import { useDownloader } from '@/composables/downloader.js'

const props = defineProps(['album'])
const emit = defineEmits(['back', 'playTrack', 'downloadLyrics'])

const { addToQueue } = useDownloader()

const trackIds = ref([])
const parentRef = ref(null)
const coverImageUrl = ref(null)
const lastfmLinksEnabled = ref(true)

const rowVirtualizer = useVirtualizer(
  computed(() => ({
    count: trackIds.value.length,
    getScrollElement: () => parentRef.value,
    estimateSize: () => 52,
    overscan: 5,
    paddingStart: 300,
    getItemKey: (index) => trackIds.value[index]
  }))
)

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems())

const totalSize = computed(() => rowVirtualizer.value.getTotalSize())

const lastfmAlbumUrl = computed(() => {
  if (!props.album?.artist_name || !props.album?.name || !lastfmLinksEnabled.value) {
    return null
  }

  const encodedArtist = encodeURIComponent(props.album.artist_name)
  const encodedAlbum = encodeURIComponent(props.album.name)
  return `https://www.last.fm/music/${encodedArtist}/${encodedAlbum}`
})

const downloadLyrics = (track) => {
  emit('downloadLyrics', track)
}

const loadCoverImage = async () => {
  if (!props.album?.image_path) {
    coverImageUrl.value = null
    return
  }

  try {
    const imageData = await invoke('get_cover_image', { imagePath: props.album.image_path })
    const blob = new Blob([new Uint8Array(imageData)], { type: 'image/jpeg' })
    coverImageUrl.value = URL.createObjectURL(blob)
  } catch (error) {
    console.error('Failed to load cover image:', error)
    coverImageUrl.value = null
  }
}

const downloadAlbumLyrics = async () => {
  const config = await invoke('get_config')
  const downloadTrackIds = await invoke('get_album_track_ids', {
    albumId: props.album.id,
    withoutPlainLyrics: config.skip_tracks_with_plain_lyrics,
    withoutSyncedLyrics: config.skip_tracks_with_synced_lyrics
  })
  addToQueue(downloadTrackIds)
}

onMounted(async () => {
  try {
    const config = await invoke('get_config')
    lastfmLinksEnabled.value = config.lastfm_links_enabled
  } catch (error) {
    console.error('Failed to load config:', error)
  }

  trackIds.value = await invoke('get_album_track_ids', { albumId: props.album.id })
  loadCoverImage()
})

onUnmounted(() => {
  if (coverImageUrl.value) {
    URL.revokeObjectURL(coverImageUrl.value)
  }
})
</script>
