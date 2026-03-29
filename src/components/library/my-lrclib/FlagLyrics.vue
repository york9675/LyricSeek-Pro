<template>
  <BaseModal
    content-class="max-w-[500px] max-h-[60vh]"
    @close="emit('close')"
    :title="`${props.track.name} - ${props.track.artistName}`"
    body-class="flex flex-col h-full min-h-0"
    :click-to-close="!isFlagging"
    :esc-to-close="!isFlagging"
    :closeButton="!isFlagging"
  >
    <div class="flex flex-col items-center">
      <div v-if="!isFlagging">
        <div class="mb-4">{{ t('flagLyrics.confirmQuestion', { track: `${track.name} - ${track.artistName}` }) }}</div>

        <label for="flagReason" class="mb-2 text-xs font-bold">{{ t('flagLyrics.explainLabel') }}</label>
        <textarea id="flagReason" v-model="flagReason" class="w-full p-2 rounded textarea" :placeholder="t('flagLyrics.explainPlaceholder')" />
      </div>
      <div v-else class="mb-4">{{ t('flagLyrics.inProgressMessage', { track: `${track.name} - ${track.artistName}` }) }}</div>

      <table v-if="isFlagging" class="text-xs table-auto font-mono uppercase">
        <tbody>
          <tr>
            <td class="px-2 py-1">{{ t('flagLyrics.requestChallenge') }}</td>
            <td class="text-right px-2 py-1">{{ progress.requestChallenge }}</td>
          </tr>

          <tr>
            <td class="px-2 py-1">{{ t('flagLyrics.solveChallenge') }}</td>
            <td class="text-right px-2 py-1">{{ progress.solveChallenge }}</td>
          </tr>

          <tr>
            <td class="px-2 py-1">{{ t('flagLyrics.flagLyrics') }}</td>
            <td class="text-right px-2 py-1">{{ progress.flagLyrics }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <template #footer>
      <div v-if="!isFlagging" class="flex gap-2 justify-center w-full">
        <button class="button button-primary px-8 py-2 rounded-full" @click="flagLyrics">{{ t('flagLyrics.confirm') }}</button>
        <button class="button button-normal px-8 py-2 rounded-full" @click="$emit('close')">{{ t('common.cancel') }}</button>
      </div>

      <div v-else class="flex gap-2 justify-center w-full">
        <button class="button button-disabled px-8 py-2 rounded-full flex gap-3" disabled>
          <Loading class="animate-spin" />
          <div>{{ t('flagLyrics.flagging') }}</div>
        </button>
      </div>
    </template>
  </BaseModal>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loading } from 'mdue'
import { listen } from '@tauri-apps/api/event'
import { useToast } from 'vue-toastification'

const toast = useToast()
const { t } = useI18n()
const emit = defineEmits(['close'])
const props = defineProps(['track'])

const isFlagging = ref(false)
const isError = ref(false)
const flagReason = ref('')

const progress = ref({
  requestChallenge: 'Pending',
  solveChallenge: 'Pending',
  flagLyrics: 'Pending'
})

const flagLyrics = async () => {
  isFlagging.value = true

  try {
    await invoke('flag_lyrics', { trackId: props.track.id, flagReason: flagReason.value })
    toast.success(t('flagLyrics.success'))
  } catch (error) {
    isError.value = true
    console.error(error)
    toast.error(error)
  } finally {
    isFlagging.value = false
    emit('close')
  }
}

onMounted(() => {
  listen('flag-lyrics-progress', (event) => {
    progress.value = event.payload
  })
})
</script>
