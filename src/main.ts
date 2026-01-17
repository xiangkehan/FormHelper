import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import naive from 'naive-ui'
import App from './App.vue'
import router from './router'

// 导入语言文件
import zhCN from './locales/zh-CN.json'
import enUS from './locales/en-US.json'

// i18n 配置
const i18n = createI18n({
  legacy: false,
  locale: localStorage.getItem('language') || 'zh-CN',
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
})

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(i18n)
app.use(naive)

// 预加载数据
import { usePersonStore } from '@/stores/personStore'
import { useFileStore } from '@/stores/fileStore'

usePersonStore().fetchPersons()
useFileStore().fetchFiles()

app.mount('#app')
