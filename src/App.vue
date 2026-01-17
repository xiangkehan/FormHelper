<template>
  <n-config-provider :theme="theme" :theme-overrides="themeOverrides">
    <n-message-provider>
      <n-dialog-provider>
        <n-notification-provider>
          <n-loading-bar-provider>
            <div class="app-container">
              <!-- 顶部导航栏 -->
              <header class="header">
                <div class="header-left">
                  <div class="logo">
                    <n-icon size="24" color="#18a058">
                      <TableDocument24Regular />
                    </n-icon>
                    <span class="logo-text">{{ $t('app.title') }}</span>
                  </div>
                </div>
                <nav class="header-nav">
                  <n-menu
                    v-model:value="activeKey"
                    mode="horizontal"
                    :options="menuOptions"
                    @update:value="handleMenuClick"
                  />
                </nav>
                <div class="header-right">
                  <n-select
                    v-model:value="currentLang"
                    :options="languageOptions"
                    size="small"
                    style="width: 120px"
                    @update:value="handleLanguageChange"
                  />
                </div>
              </header>

              <!-- 主内容区域 -->
              <main class="main-content">
                <router-view />
              </main>
            </div>
          </n-loading-bar-provider>
        </n-notification-provider>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NIcon } from 'naive-ui'
import { TableDocument24Regular, Person24Regular, Folder24Regular, Settings24Regular } from '@vicons/fluent'
import { useI18n } from 'vue-i18n'

// 路由
const router = useRouter()
const route = useRoute()
const { locale } = useI18n()

// 主题
const isDark = ref(false)
const theme = computed(() => isDark.value ? undefined : null)

// 主题配置
const themeOverrides = {
  common: {
    primaryColor: '#18a058',
    primaryColorHover: '#36ad6a',
    primaryColorPressed: '#0c7a43'
  }
}

// 菜单选项
const menuOptions = [
  {
    label: '首页',
    key: 'home',
    icon: () => h(NIcon, null, { default: () => h(TableDocument24Regular) })
  },
  {
    label: '人员管理',
    key: 'persons',
    icon: () => h(NIcon, null, { default: () => h(Person24Regular) })
  },
  {
    label: '文件处理',
    key: 'files',
    icon: () => h(NIcon, null, { default: () => h(Folder24Regular) })
  },
  {
    label: '设置',
    key: 'settings',
    icon: () => h(NIcon, null, { default: () => h(Settings24Regular) })
  }
]

const languageOptions = [
  { label: '简体中文', value: 'zh-CN' },
  { label: 'English', value: 'en-US' }
]

const activeKey = ref('home')
const currentLang = ref(localStorage.getItem('language') || 'zh-CN')

// 初始化激活菜单
activeKey.value = route.name as string || 'home'

// 菜单点击处理
const handleMenuClick = (key: string) => {
  router.push({ name: key })
}

// 语言切换
const handleLanguageChange = (value: string) => {
  localStorage.setItem('language', value)
  locale.value = value
}
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  height: 60px;
  background: var(--n-color);
  border-bottom: 1px solid var(--n-border-color);
}

.header-left {
  display: flex;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: var(--n-text-color);
}

.header-nav {
  flex: 1;
  display: flex;
  justify-content: center;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.main-content {
  flex: 1;
  padding: 24px;
  background: var(--n-color);
}
</style>
