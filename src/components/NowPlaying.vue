<template>
  <div>
    <LyricsViewer v-if="lyrics && !instrumental" :lyrics="lyrics" :duration="duration" :progress="progress" @lyrics-clicked="lyricsClicked" />
    <PlainLyricsViewer v-else-if="plainLyrics && !instrumental" :lyrics="plainLyrics" />
    <div v-else class="border-b border-brave-90/50 dark:border-brave-30"></div>
    <div class="bg-brave-95 backdrop-blur px-4 py-3 flex-none flex flex-col justify-center items-center gap-3 dark:bg-brave-10">
      <div class="w-full flex gap-1 justify-center items-center">
        <div class="flex-none w-12 text-xs text-brave-30 dark:text-brave-80">{{ humanDuration(progress) }}</div>
        <Seek class="grow" :duration="duration" :progress="progress" @seek="seek" />
        <div class="flex-none text-right w-12 text-xs text-brave-30 dark:text-brave-80">{{ humanDuration(duration) }}</div>
      </div>

      <div class="flex justify-between w-full items-center gap-3">
        <!-- Cover art in player bar (left side) -->
        <div class="basis-1/3 flex-1 grow-0 flex items-center gap-2">
          <div v-if="coverImageUrl" class="w-12 h-12 rounded overflow-hidden shadow flex-shrink-0">
            <img :src="coverImageUrl" :alt="playingTrack?.album_name" class="w-full h-full object-cover">
          </div>
          <div v-else class="w-12 h-12 rounded bg-brave-90 dark:bg-brave-20 flex items-center justify-center flex-shrink-0">
            <div class="text-sm">🎵</div>
          </div>
          
          <!-- Title and artist info -->
          <div class="flex flex-col justify-center items-start gap-0.5 min-w-0 flex-1">
            <div v-if="playingTrack">
              <div class="text-xs font-bold text-brave-30 dark:text-brave-95 line-clamp-1">
                <a v-if="lastfmLinksEnabled && lastfmAlbumUrl" :href="lastfmAlbumUrl" target="_blank" rel="noopener noreferrer" class="link hover:underline">{{ playingTrack.title }}</a>
                <span v-else>{{ playingTrack.title }}</span>
              </div>
              <div class="text-xs text-brave-40 dark:text-brave-90 line-clamp-1">
                <a v-if="lastfmLinksEnabled && lastfmAlbumUrl" :href="lastfmAlbumUrl" target="_blank" rel="noopener noreferrer" class="link hover:underline">{{ playingTrack.album_name }}</a>
                <span v-else>{{ playingTrack.album_name }}</span>
                <span> - </span>
                <a v-if="lastfmLinksEnabled && lastfmArtistUrl" :href="lastfmArtistUrl" target="_blank" rel="noopener noreferrer" class="link hover:underline">{{ playingTrack.artist_name }}</a>
                <span v-else>{{ playingTrack.artist_name }}</span>
              </div>
            </div>
          </div>
        </div>

        <div class="basis-1/3 flex-1 flex justify-center items-center gap-2">
          <button @click.prevent="seek(reverse10)" class="button button-secondary p-1 m-1 rounded-full text-lg"><Rewind_10 /></button>
          <button v-if="status === 'playing'" @click.prevent="pause" class="button button-primary text-white p-2 rounded-full text-xl"><Pause /></button>
          <button v-else-if="playingTrack && status === 'stopped'" @click.prevent="playTrack(playingTrack)" class="button button-primary text-white p-2 rounded-full text-xl"><Replay /></button>
          <button v-else @click.prevent="resume" class="button button-primary text-white p-2 rounded-full text-xl"><Play /></button>
          <button @click.prevent="seek(forward10)" class="button button-secondary p-1 m-1 rounded-full text-lg"><FastForward_10 /></button>
        </div>

        <div class="basis-1/3 flex-1 flex justify-end items-center">
          <VolumeSlider :volume="volume" @set-volume="setPlayerVolume" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from '@vue/reactivity'
import { ref, onMounted, onUnmounted, watch } from 'vue'
import Seek from './now-playing/Seek.vue'
import LyricsViewer from './now-playing/LyricsViewer.vue'
import PlainLyricsViewer from './now-playing/PlainLyricsViewer.vue'
import { Play, Pause, Replay, Rewind_10, FastForward_10 } from 'mdue'
import { usePlayer } from '@/composables/player.js'
import { useGlobalState } from '@/composables/global-state.js'
import VolumeSlider from './now-playing/VolumeSlider.vue'
import { humanDuration } from '@/utils/human-duration'
import { invoke } from '@tauri-apps/api/core'

const { isHotkey } = useGlobalState()
const { playingTrack, status, duration, progress, volume, playTrack, pause, resume, seek, setVolume: setPlayerVolume } = usePlayer()
const keydownEvent = ref(null)
const coverImageUrl = ref(null)
const lastfmLinksEnabled = ref(true)

const instrumental = computed(() => {
  if (!playingTrack.value) {
    return null
  }

  return playingTrack.value.instrumental
})

const lyrics = computed(() => {
  if (!playingTrack.value) {
    return null
  }

  return playingTrack.value.lrc_lyrics
})

const plainLyrics = computed(() => {
  if (!playingTrack.value) {
    return null
  }

  return playingTrack.value.txt_lyrics
})

const forward10 = computed(() => {
  if (!playingTrack.value) {
    return null
  }
  return Math.min(duration.value - 1, progress.value + 10)
})

const reverse10 = computed(() => {
  if (!playingTrack.value) {
    return null
  }
  return Math.max(0, progress.value - 10)
})

const lastfmArtistUrl = computed(() => {
  if (!playingTrack.value?.artist_name || !lastfmLinksEnabled.value) {
    return null
  }
  const encodedArtist = encodeURIComponent(playingTrack.value.artist_name)
  return `https://www.last.fm/music/${encodedArtist}`
})

const lastfmAlbumUrl = computed(() => {
  if (!playingTrack.value?.album_name || !playingTrack.value?.artist_name || !lastfmLinksEnabled.value) {
    return null
  }
  const encodedArtist = encodeURIComponent(playingTrack.value.artist_name)
  const encodedAlbum = encodeURIComponent(playingTrack.value.album_name)
  return `https://www.last.fm/music/${encodedArtist}/${encodedAlbum}`
})

const setVolume = (event) => {
  const volume = parseFloat(event.target.value)
  setPlayerVolume(volume)
}

const lyricsClicked = (line) => {
  seek(line.timestamp)
}

const loadCoverImage = async () => {
  if (!playingTrack.value?.image_path) {
    coverImageUrl.value = null
    return
  }

  try {
    const imageData = await invoke('get_cover_image', { imagePath: playingTrack.value.image_path })
    const blob = new Blob([new Uint8Array(imageData)], { type: 'image/jpeg' })
    coverImageUrl.value = URL.createObjectURL(blob)
  } catch (error) {
    console.error('Failed to load cover image:', error)
    coverImageUrl.value = null
  }
}

watch(() => playingTrack.value?.id, () => {
  loadCoverImage()
})

onUnmounted(async () => {
  if (keydownEvent.value) {
    document.removeEventListener(keydownEvent.value)
  }
  if (coverImageUrl.value) {
    URL.revokeObjectURL(coverImageUrl.value)
  }
})

onMounted(async () => {
  try {
    const config = await invoke('get_config')
    lastfmLinksEnabled.value = config.lastfm_links_enabled
  } catch (error) {
    console.error('Failed to load config:', error)
  }

  keydownEvent.value = document.addEventListener('keydown', (event) => {
    if (!isHotkey.value) {
      // hotkey is explicitly disabled
      return
    }

    const target = event.target

    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.closest('.v-codemirror')) {
      // Do nothing if the target is an input, textarea, or CodeMirror element
      return
    }

    switch (event.code) {
      case 'Space':
      case 'Enter':
        event.preventDefault()
        if (status.value==='playing') {
          pause()
        } else if (playingTrack.value && status.value==='stopped') {
          playTrack(playingTrack.value)
        } else {
          resume()
        }
        break;
      case 'ArrowLeft':
        event.preventDefault()
        seek(reverse10.value)
        break;
      case 'ArrowRight':
        event.preventDefault()
        seek(forward10.value)
        break;
      default:
        break;
    }
  })
  
  loadCoverImage()
})
</script>
