import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  const isDark = ref(false)
  const language = ref(localStorage.getItem('language') || 'zh-CN')

  const toggleTheme = () => {
    isDark.value = !isDark.value
  }

  const setLanguage = (lang: string) => {
    language.value = lang
    localStorage.setItem('language', lang)
  }

  return {
    isDark,
    language,
    toggleTheme,
    setLanguage
  }
})
