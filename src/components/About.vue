<template>
  <BaseModal
    :title="t('about.title')"
    @opened="openedHandler"
    @close="emit('close')"
  >
    <div class="overflow-auto text-sm">
      <div class="mb-6">
        <label class="group-label mb-2">{{ t('about.project') }}</label>
        <div class="mb-1"><span class="font-bold">LyricSeek Pro</span></div>
        <div>
          {{ t('about.description') }}
          <br>
          <br>
          {{ t('about.modifiedFromPrefix') }} <a href="https://github.com/tranxuanthang/lrcget" class="link" target="_blank">tranxuanthang/lrcget</a>.
        </div>
        <div>
        <details class="rounded-lg border border-brave-90 dark:border-brave-20 bg-brave-98 dark:bg-brave-5 p-3">
          <summary class="cursor-pointer font-semibold">{{ t('about.licenseSummary') }}</summary>
          <pre class="whitespace-pre-wrap text-xs mt-3 max-h-72 overflow-auto">
MIT License

Copyright (c) 2023 tranxuanthang

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
          </pre>
        </details>
      </div>
      </div>

      <div class="mb-6">
        <label class="group-label mb-2">{{ t('about.update') }}</label>

        <div class="text-sm">
          <div>
            {{ t('about.repository') }}: <a href="https://github.com/york9675/LyricSeek-Pro" class="link" target="_blank">LyricSeek-Pro</a>
          </div>
          <div class="mb-1">{{ t('about.currentVersion') }}: <span class="font-bold">{{ version }}</span></div>
          <template v-if="!isCheckingForUpdate && newestVersion">
            <div class="mb-1">{{ t('about.newestVersion') }}: <span class="font-bold">{{ newestVersion }}</span></div>
            <div v-if="isUpdateAvailable" class="font-bold text-yellow-600 flex flex-wrap gap-1 items-center">
              <Alert class="text-lg" />
              {{ t('about.updateAvailable') }}
              <a :href="newestReleaseUrl" class="link" target="_blank" rel="noopener">{{ t('about.viewReleasePage') }}</a>
            </div>
            <div v-else class="font-bold text-green-600 flex flex-wrap gap-1 items-center">
              <CheckCircle class="text-lg" />
              {{ t('about.latestVersion') }}
            </div>
          </template>
          <div v-else-if="!isCheckingForUpdate && !newestVersion">
            {{ t('about.updateCheckFailed') }}
          </div>
          <div v-else>
            {{ t('about.checkingUpdate') }}
          </div>
        </div>
      </div>
      
      <div class="mb-6">
        <label class="group-label mb-2">{{ t('about.developer') }}</label>
        <div class="mb-1">{{ t('about.developer') }}: <a href="https://github.com/york9675/" class="link" target="_blank">york9675</a></div>
        <div class="mb-1">
          {{ t('about.contact') }}: <a href="mailto:york@york.qzz.io" class="link" target="_blank">york@york.qzz.io</a>
        </div>
      </div>

      <div class="mb-6">
        <label class="group-label mb-2">{{ t('about.support') }}</label>
        <a href="https://buymeacoffee.com/york0524" class="link" target="_blank">https://buymeacoffee.com/york0524</a>
      </div>

    </div>
  </BaseModal>
</template>

<script setup>
import { Alert, CheckCircle } from 'mdue'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { getVersion } from '@tauri-apps/api/app'
import semver from 'semver'
import BaseModal from '@/components/common/BaseModal.vue'

const emit = defineEmits(['close'])
const { t } = useI18n()

const version = ref('1.0.0')
const newestVersion = ref(null)
const newestReleaseUrl = ref('https://github.com/york9675/LyricSeek-Pro/releases')
const isCheckingForUpdate = ref(false)

const normalizeVersion = (value) => {
  const coerced = semver.coerce(value)
  return coerced ? coerced.version : null
}

const isUpdateAvailable = computed(() => {
  const current = normalizeVersion(version.value)
  const latest = normalizeVersion(newestVersion.value)
  if (!current || !latest) {
    return false
  }
  return semver.gt(latest, current)
})

async function getLatestReleaseInfo() {
  const repo = 'york9675/LyricSeek-Pro'
  const apiUrl = `https://api.github.com/repos/${repo}/releases/latest`

  const response = await fetch(apiUrl)
  if (!response.ok) {
    throw new Error(`Error: ${response.status}`)
  }
  const data = await response.json()
  return {
    version: data.tag_name,
    releaseUrl: data.html_url || `https://github.com/${repo}/releases`
  }
}

const openedHandler = async () => {
  isCheckingForUpdate.value = true
  try {
    version.value = await getVersion()

    const latestReleaseInfo = await getLatestReleaseInfo()
    newestVersion.value = latestReleaseInfo.version
    newestReleaseUrl.value = latestReleaseInfo.releaseUrl
  } catch (error) {
    console.error(error)
    newestVersion.value = null
  } finally {
    isCheckingForUpdate.value = false
  }
}
</script>
