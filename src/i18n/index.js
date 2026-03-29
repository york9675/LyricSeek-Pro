import { createI18n } from 'vue-i18n'
import en from './locales/en.js'
import zhTw from './locales/zh-TW.js'

const LOCALE_STORAGE_KEY = 'lyricseekpro-locale'
const SYSTEM_LOCALE = 'system'

const normalizeLocale = (locale) => {
  const normalized = String(locale || '').toLowerCase()
  if (normalized === 'zh-tw' || normalized === 'zh-hant' || normalized === 'zh-hk' || normalized === 'zh-mo') {
    return 'zh-TW'
  }
  return 'en'
}

const getSystemLocale = () => {
  const [browserLocale] = navigator.languages || [navigator.language]
  return normalizeLocale(browserLocale)
}

const resolveLocale = (localePreference) => {
  if (localePreference === SYSTEM_LOCALE) {
    return getSystemLocale()
  }
  return normalizeLocale(localePreference)
}

const getInitialLocale = () => {
  const localePreference = localStorage.getItem(LOCALE_STORAGE_KEY)
  if (localePreference) {
    return resolveLocale(localePreference)
  }

  return getSystemLocale()
}

export const i18n = createI18n({
  legacy: false,
  locale: getInitialLocale(),
  fallbackLocale: 'en',
  globalInjection: true,
  messages: {
    en,
    'zh-TW': zhTw
  }
})

export const getLocalePreference = () => {
  const storedLocale = localStorage.getItem(LOCALE_STORAGE_KEY)
  if (storedLocale === SYSTEM_LOCALE) {
    return SYSTEM_LOCALE
  }
  return normalizeLocale(storedLocale)
}

export const syncNativeLocale = async (localePreference = SYSTEM_LOCALE) => {
  const requestedLocale = localePreference === SYSTEM_LOCALE
    ? SYSTEM_LOCALE
    : normalizeLocale(localePreference)

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('set_app_locale', { locale: requestedLocale })
  } catch (_error) {
    // Ignore when running without Tauri backend.
  }
}

export const setLocale = (localePreference) => {
  const nextPreference = localePreference === SYSTEM_LOCALE
    ? SYSTEM_LOCALE
    : normalizeLocale(localePreference)
  const resolvedLocale = resolveLocale(nextPreference)
  i18n.global.locale.value = resolvedLocale
  localStorage.setItem(LOCALE_STORAGE_KEY, nextPreference)

  // Native macOS menu/file dialogs use system locale for consistency.
  syncNativeLocale(SYSTEM_LOCALE)
}
