import { ref, computed } from 'vue'

const isHotkeyState = ref(true)
const themeModeState = ref('auto')
const lrclibInstanceState = ref('')
const showCoverArtInTrackListState = ref(true)

export function useGlobalState() {
  const disableHotkey = () => {
    console.log('disabled hotkey!')
    isHotkeyState.value = false
  }
  const enableHotkey = () => {
    console.log('enabled hotkey!')
    isHotkeyState.value = true
  }
  const isHotkey = computed(() => isHotkeyState.value)

  const setThemeMode = (mode) => {
    themeModeState.value = mode
  }
  const setLrclibInstance = (instance) => {
    lrclibInstanceState.value = instance
  }
  const setShowCoverArtInTrackList = (isEnabled) => {
    showCoverArtInTrackListState.value = isEnabled
  }

  const lrclibInstance = computed(() => lrclibInstanceState.value)
  const showCoverArtInTrackList = computed(() => showCoverArtInTrackListState.value)

  const themeMode = computed(() => themeModeState.value)

  return {
    isHotkey,
    disableHotkey,
    enableHotkey,
    setThemeMode,
    themeMode,
    setLrclibInstance,
    lrclibInstance,
    setShowCoverArtInTrackList,
    showCoverArtInTrackList
  }
}
