<template>
  <div class="flex w-full group hover:bg-brave-98 hover:shadow hover:shadow-brave-95/50
    border hover:border-brave-95 transition rounded cursor-default
    dark:hover:bg-brave-10 dark:hover:border-brave-20 dark:hover:shadow-brave-30/50"
    :class="{
      'border-brave-95 bg-brave-99 dark:border-brave-20 dark:bg-brave-5': isPlaying,
      'border-brave-90 bg-brave-98 dark:border-brave-30 dark:bg-brave-10': isContextSelected && !isPlaying,
      'border-transparent': !isPlaying && !isContextSelected
      }"
    @contextmenu.prevent.stop="openContextMenu"
  >
    <!-- Cover thumbnail -->
    <div v-if="showCover" class="flex-none w-12 h-12 flex items-center justify-center p-1 flex-shrink-0">
      <div v-if="coverImageUrl" class="w-10 h-10 rounded overflow-hidden">
        <img :src="coverImageUrl" :alt="track?.album_name" class="w-full h-full object-cover">
      </div>
      <div v-else class="w-10 h-10 rounded bg-brave-90 dark:bg-brave-20 flex items-center justify-center">
        <div class="text-xs text-brave-50 dark:text-brave-70">🎵</div>
      </div>
    </div>

    <!-- Track number -->
    <div
      v-if="isShowTrackNumber"
      class="flex-none w-[5%] flex items-center justify-end p-1 pr-2 text-xs text-brave-30/70 dark:text-brave-99 font-bold"
    >
      <div v-if="track && track.track_number">{{ track.track_number }}</div>
      <div v-else>--</div>
    </div>

    <!-- Track title, album, and artist -->
    <div
      class="flex-none flex p-1"
      :class="{
        'w-[calc(65%-3rem)]': !isShowTrackNumber && showCover,
        'w-[calc(60%-3rem)]': isShowTrackNumber && showCover,
        'w-[65%]': !isShowTrackNumber && !showCover,
        'w-[60%]': isShowTrackNumber && !showCover
      }"
      @click="playTrack(track)"
    >
      <div v-if="track">
        <div class="font-bold text-sm text-brave-20 flex items-center dark:text-brave-95">
          <Equalizer v-if="isPlaying && status === 'playing' && !editingTrack" class="mr-1" />
          <span
            v-if="useLastfmLinks && lastfmTrackUrl"
            class="link cursor-pointer"
            @click.stop="openExternalLink(lastfmTrackUrl)"
          >
            {{ track.title }}
          </span>
          <span v-else>{{ track.title }}</span>
        </div>

        <div class="gap-2 line-clamp-1">
          <span
            v-if="useLastfmLinks && lastfmAlbumUrl"
            class="text-sm link"
            @click.stop="openExternalLink(lastfmAlbumUrl)"
          >
            {{ track.album_name }}
          </span>
          <span
            v-else-if="useInAppLinks"
            class="text-sm link cursor-pointer"
            @click.stop="openAlbumPage"
          >
            {{ track.album_name }}
          </span>
          <span v-else class="text-sm text-brave-30 transition dark:text-brave-90">{{ track.album_name }}</span>
          <span class="text-brave-30 h-full mx-1 flex-none dark:text-brave-90">|</span>
          <span
            v-if="useLastfmLinks && lastfmArtistUrl"
            class="text-sm link"
            @click.stop="openExternalLink(lastfmArtistUrl)"
          >
            {{ track.artist_name }}
          </span>
          <span
            v-else-if="useInAppLinks"
            class="text-sm link cursor-pointer"
            @click.stop="openArtistPage"
          >
            {{ track.artist_name }}
          </span>
          <span v-else class="text-sm text-brave-30 transition dark:text-brave-90">{{ track.artist_name }}</span>
        </div>
      </div>
    </div>

    <!-- Duration -->
    <div class="flex-none w-[10%] flex items-center justify-end p-1" @click="playTrack(track)">
      <div v-if="track" class="text-brave-30 font-bold text-xs text-right dark:text-brave-95">{{ humanDuration(track.duration) }}</div>
    </div>

    <!-- Lyrics indication -->
    <div class="flex-none w-[10%] flex items-center justify-center p-1" @click="playTrack(track)">
      <div v-if="track">
        <span v-if="track.instrumental" class="text-gray-200 font-bold text-[0.67rem] bg-gray-500 rounded px-1 py-0.5">Instrumental</span>
        <span v-else-if="track.lrc_lyrics" class="text-green-200 font-bold text-[0.67rem] bg-green-800 rounded px-1 py-0.5">Synced</span>
        <span v-else-if="track.txt_lyrics" class="text-gray-200 font-bold text-[0.67rem] bg-gray-800 rounded px-1 py-0.5">Plain</span>
      </div>
    </div>

    <!-- Action buttons -->
    <div class="flex-none w-[15%] h-full flex justify-end items-center p-1">
      <div v-if="track" class="flex justify-end items-center gap-1">
        <button v-if="isPlaying && status ==='playing'" @click="pause" class="track-button"><Pause /></button>
        <button v-else-if="isPlaying && status === 'stopped'" @click="playTrack(track)" class="track-button"><Replay /></button>
        <button v-else @click="isPlaying ? resume() : playTrack(track)" class="track-button"><Play /></button>
        <button class="track-button" @click="searchLyrics(track)"><TextSearch /></button>
        <button class="track-button" @click="editLyrics(track)"><PlaylistEdit /></button>
      </div>
    </div>
  </div>

  <Teleport to="body">
    <div
      v-if="contextMenuVisible && track"
      ref="contextMenuRef"
      class="fixed z-[70] min-w-[13rem] rounded bg-brave-95 dark:bg-brave-5 shadow shadow-brave-80 dark:shadow-brave-10 p-1"
      :style="{ top: `${contextMenuY}px`, left: `${contextMenuX}px` }"
      @click.stop
    >
      <button class="context-item" @click="playFromContext">
        <Play class="text-base" />
        <span>Play</span>
      </button>
      <button class="context-item" @click="searchFromContext">
        <TextSearch class="text-base" />
        <span>Search lyrics</span>
      </button>
      <button class="context-item" @click="editFromContext">
        <PlaylistEdit class="text-base" />
        <span>Edit lyrics</span>
      </button>

      <div class="context-divider"></div>

      <div
        class="relative"
        @mouseenter="openGotoMenu"
        @mouseleave="closeGotoMenu"
      >
        <button class="context-item w-full">
          <FolderMultiple class="text-base" />
          <span class="grow text-left">Goto</span>
          <span class="text-xs opacity-70">▶</span>
        </button>

        <div
          v-if="gotoMenuOpen"
          ref="gotoMenuRef"
          class="absolute min-w-[13rem] rounded bg-brave-95 dark:bg-brave-5 shadow shadow-brave-80 dark:shadow-brave-10 p-1"
          :class="gotoMenuDirection === 'right' ? 'top-0 left-full ml-1' : 'top-0 right-full mr-1'"
          :style="{ top: `${gotoMenuOffsetY}px` }"
        >
          <div class="px-2 py-1 text-[0.65rem] uppercase tracking-wide text-brave-40 dark:text-brave-70">In app</div>
          <button class="context-item" @click="gotoInAppAlbum">
            <Music class="text-base" />
            <span>Album</span>
          </button>
          <button class="context-item" @click="gotoInAppArtist">
            <Music class="text-base" />
            <span>Artist</span>
          </button>

          <div class="context-divider"></div>

          <div class="px-2 py-1 text-[0.65rem] uppercase tracking-wide text-brave-40 dark:text-brave-70">Last.fm</div>
          <button class="context-item" @click="gotoLastfmTrack">
            <OpenInNew class="text-base" />
            <span>Track</span>
          </button>
          <button class="context-item" @click="gotoLastfmAlbum">
            <OpenInNew class="text-base" />
            <span>Album</span>
          </button>
          <button class="context-item" @click="gotoLastfmArtist">
            <OpenInNew class="text-base" />
            <span>Artist</span>
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { Play, Pause, TextSearch, PlaylistEdit, Replay, FolderMultiple, OpenInNew, Music } from 'mdue'
import { humanDuration } from '../../../utils/human-duration.js'
import { useSearchLyrics } from '../../../composables/search-lyrics.js'
import { useEditLyrics } from '../../../composables/edit-lyrics.js'
import Equalizer from '@/components/icons/Equalizer.vue'
import { ref, onMounted, computed, watch, onUnmounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen as listenEvent } from '@tauri-apps/api/event'
import { open as openShell } from '@tauri-apps/plugin-shell'
import { usePlayer } from '@/composables/player.js'

const { playTrack, playingTrack, status, pause, resume } = usePlayer()

const { searchLyrics } = useSearchLyrics()
const { editLyrics, editingTrack } = useEditLyrics()
const props = defineProps({
  trackId: {
    type: Number,
    required: true
  },
  isShowTrackNumber: {
    type: Boolean,
    default: false
  },
  showCover: {
    type: Boolean,
    default: true
  }
})
const track = ref(null)
const coverImageUrl = ref(null)
const showCover = computed(() => props.showCover !== false)
const showLinks = ref(true)
const linksTarget = ref('in_app')
const contextMenuVisible = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const gotoMenuOpen = ref(false)
const contextMenuRef = ref(null)
const gotoMenuRef = ref(null)
const gotoMenuDirection = ref('right')
const gotoMenuOffsetY = ref(0)

const isContextSelected = computed(() => contextMenuVisible.value)

const isPlaying = computed(() => {
  return playingTrack.value && track.value && playingTrack.value.id === track.value.id
})

const useLastfmLinks = computed(() => showLinks.value && linksTarget.value === 'lastfm')
const useInAppLinks = computed(() => showLinks.value && linksTarget.value === 'in_app')

const lastfmAlbumUrl = computed(() => {
  if (!track.value?.artist_name || !track.value?.album_name || !useLastfmLinks.value) {
    return null
  }
  const encodedArtist = encodeURIComponent(track.value.artist_name)
  const encodedAlbum = encodeURIComponent(track.value.album_name)
  return `https://www.last.fm/music/${encodedArtist}/${encodedAlbum}`
})

const lastfmArtistUrl = computed(() => {
  if (!track.value?.artist_name || !useLastfmLinks.value) {
    return null
  }
  const encodedArtist = encodeURIComponent(track.value.artist_name)
  return `https://www.last.fm/music/${encodedArtist}`
})

const lastfmTrackUrl = computed(() => {
  if (!track.value?.artist_name || !track.value?.title) {
    return null
  }
  const encodedArtist = encodeURIComponent(track.value.artist_name)
  const encodedTitle = encodeURIComponent(track.value.title)
  return `https://www.last.fm/music/${encodedArtist}/_/${encodedTitle}`
})

const applyConfig = (config) => {
  showLinks.value = config?.show_links ?? config?.showLinks ?? true
  linksTarget.value = config?.links_target || config?.linksTarget || 'in_app'
}

const openAlbumPage = () => {
  if (!track.value?.album_name || !track.value?.artist_name) {
    return
  }

  window.dispatchEvent(new CustomEvent('now-playing-open-library-target', {
    detail: {
      type: 'album',
      albumName: track.value.album_name,
      artistName: track.value.artist_name
    }
  }))
}

const openArtistPage = () => {
  if (!track.value?.artist_name) {
    return
  }

  window.dispatchEvent(new CustomEvent('now-playing-open-library-target', {
    detail: {
      type: 'artist',
      artistName: track.value.artist_name
    }
  }))
}

const handleConfigUpdated = (event) => {
  applyConfig(event?.detail)
}

const openContextMenu = (event) => {
  if (!track.value) {
    return
  }

  window.dispatchEvent(new CustomEvent('track-context-opened', {
    detail: {
      trackId: props.trackId
    }
  }))

  const viewportPadding = 8
  const estimatedWidth = 220
  const estimatedHeight = 260

  contextMenuX.value = Math.min(
    Math.max(event.clientX, viewportPadding),
    window.innerWidth - estimatedWidth - viewportPadding
  )
  contextMenuY.value = Math.min(
    Math.max(event.clientY, viewportPadding),
    window.innerHeight - estimatedHeight - viewportPadding
  )
  contextMenuVisible.value = true

  nextTick(() => {
    const menuEl = contextMenuRef.value
    if (!menuEl) {
      return
    }

    const rect = menuEl.getBoundingClientRect()
    const maxX = window.innerWidth - rect.width - viewportPadding
    const maxY = window.innerHeight - rect.height - viewportPadding

    contextMenuX.value = Math.min(Math.max(contextMenuX.value, viewportPadding), Math.max(viewportPadding, maxX))
    contextMenuY.value = Math.min(Math.max(contextMenuY.value, viewportPadding), Math.max(viewportPadding, maxY))
  })
}

const closeContextMenu = () => {
  contextMenuVisible.value = false
  closeGotoMenu()
}

const closeGotoMenu = () => {
  gotoMenuOpen.value = false
  gotoMenuDirection.value = 'right'
  gotoMenuOffsetY.value = 0
}

const openGotoMenu = () => {
  // Reset before measuring to avoid stale values causing alternating placement.
  gotoMenuDirection.value = 'right'
  gotoMenuOffsetY.value = 0
  gotoMenuOpen.value = true

  nextTick(() => {
    const menuEl = contextMenuRef.value
    const submenuEl = gotoMenuRef.value

    if (!menuEl || !submenuEl) {
      return
    }

    const viewportPadding = 8
    const menuRect = menuEl.getBoundingClientRect()
    const submenuRect = submenuEl.getBoundingClientRect()

    const spaceRight = window.innerWidth - menuRect.right
    gotoMenuDirection.value = spaceRight >= submenuRect.width + viewportPadding ? 'right' : 'left'

    let offsetY = 0
    const bottomOverflow = submenuRect.bottom - (window.innerHeight - viewportPadding)
    const topOverflow = viewportPadding - submenuRect.top

    if (bottomOverflow > 0) {
      offsetY -= bottomOverflow
    }
    if (topOverflow > 0) {
      offsetY += topOverflow
    }

    gotoMenuOffsetY.value = offsetY
  })
}

const handleTrackContextOpened = (event) => {
  const openedTrackId = event?.detail?.trackId
  if (openedTrackId !== props.trackId) {
    closeContextMenu()
  }
}

const playFromContext = () => {
  if (track.value) {
    playTrack(track.value)
  }
  closeContextMenu()
}

const searchFromContext = () => {
  if (track.value) {
    searchLyrics(track.value)
  }
  closeContextMenu()
}

const editFromContext = () => {
  if (track.value) {
    editLyrics(track.value)
  }
  closeContextMenu()
}

const openExternalLink = async (url) => {
  if (!url) {
    return
  }

  try {
    await openShell(url)
  } catch (error) {
    console.error('Failed to open external link:', error)
  }
}

const gotoInAppAlbum = () => {
  openAlbumPage()
  closeContextMenu()
}

const gotoInAppArtist = () => {
  openArtistPage()
  closeContextMenu()
}

const gotoLastfmTrack = () => {
  openExternalLink(lastfmTrackUrl.value)
  closeContextMenu()
}

const gotoLastfmAlbum = () => {
  openExternalLink(lastfmAlbumUrl.value)
  closeContextMenu()
}

const gotoLastfmArtist = () => {
  openExternalLink(lastfmArtistUrl.value)
  closeContextMenu()
}

const loadCoverImage = async () => {
  if (!showCover.value) {
    coverImageUrl.value = null
    return
  }

  if (!track.value?.image_path) {
    coverImageUrl.value = null
    return
  }

  try {
    const imageData = await invoke('get_cover_image', { imagePath: track.value.image_path })
    const blob = new Blob([new Uint8Array(imageData)], { type: 'image/jpeg' })
    coverImageUrl.value = URL.createObjectURL(blob)
  } catch (error) {
    console.error('Failed to load cover image:', error)
    coverImageUrl.value = null
  }
}

watch(() => track.value?.id, () => {
  loadCoverImage()
})

onMounted(async () => {
  const config = await invoke('get_config')
  applyConfig(config)

  track.value = await invoke('get_track', { trackId: props.trackId })
  loadCoverImage()

  window.addEventListener('config-updated', handleConfigUpdated)
  window.addEventListener('track-context-opened', handleTrackContextOpened)
  window.addEventListener('click', closeContextMenu)
  window.addEventListener('scroll', closeContextMenu, true)

  listenEvent('reload-track-id', async (event) => {
    const payload = event.payload
    if (track.value.id === payload) {
      track.value = await invoke('get_track', { trackId: props.trackId })
      loadCoverImage()
    }
  })
})

onUnmounted(() => {
  if (coverImageUrl.value) {
    URL.revokeObjectURL(coverImageUrl.value)
  }
  window.removeEventListener('config-updated', handleConfigUpdated)
  window.removeEventListener('track-context-opened', handleTrackContextOpened)
  window.removeEventListener('click', closeContextMenu)
  window.removeEventListener('scroll', closeContextMenu, true)
})
</script>

<style scoped>
.track-button {
  @apply text-brave-30 hover:bg-brave-30 hover:text-white rounded p-2 transition dark:text-white dark:hover:bg-brave-30;
}

.context-item {
  @apply flex items-center px-2 py-1 hover:bg-brave-90 dark:hover:bg-brave-15 rounded cursor-pointer h-8 gap-1 w-full text-sm text-brave-20 dark:text-brave-90;
}

.context-divider {
  @apply my-1 border-t border-brave-90 dark:border-brave-20;
}
</style>
