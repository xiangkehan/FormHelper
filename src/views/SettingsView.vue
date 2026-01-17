<template>
  <div class="settings-container">
    <n-card :title="$t('settings.title')" :bordered="false" style="max-width: 600px">
      <n-form label-placement="left" label-width="120px">
        <!-- 语言设置 -->
        <n-form-item :label="$t('settings.language')">
          <n-select
            v-model:value="settings.language"
            :options="languageOptions"
            @update:value="handleLanguageChange"
          />
        </n-form-item>

        <!-- 主题设置 -->
        <n-form-item :label="$t('settings.theme')">
          <n-radio-group v-model:value="settings.theme" @update:value="handleThemeChange">
            <n-radio value="light">{{ $t('settings.themeLight') }}</n-radio>
            <n-radio value="dark">{{ $t('settings.themeDark') }}</n-radio>
            <n-radio value="auto">{{ $t('settings.themeAuto') }}</n-radio>
          </n-radio-group>
        </n-form-item>

        <!-- OCR 引擎设置 -->
        <n-form-item :label="$t('settings.ocrEngine')">
          <n-radio-group v-model:value="settings.ocrEngine">
            <n-radio value="local">{{ $t('settings.ocrLocal') }}</n-radio>
            <n-radio value="cloud">{{ $t('settings.ocrCloud') }}</n-radio>
          </n-radio-group>
        </n-form-item>

        <!-- 保存按钮 -->
        <n-form-item>
          <n-button type="primary" @click="handleSave">
            {{ $t('settings.save') }}
          </n-button>
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'

const message = useMessage()
const { locale } = useI18n()

const languageOptions = [
  { label: '简体中文', value: 'zh-CN' },
  { label: 'English', value: 'en-US' }
]

const settings = ref({
  language: localStorage.getItem('language') || 'zh-CN',
  theme: localStorage.getItem('theme') || 'light',
  ocrEngine: localStorage.getItem('ocrEngine') || 'local'
})

const handleLanguageChange = (value: string) => {
  localStorage.setItem('language', value)
  locale.value = value
}

const handleThemeChange = (value: string) => {
  localStorage.setItem('theme', value)
  document.documentElement.classList.toggle('dark', value === 'dark')
}

const handleSave = () => {
  localStorage.setItem('settings', JSON.stringify(settings.value))
  message.success('设置已保存')
}
</script>

<style scoped>
.settings-container {
  display: flex;
  justify-content: center;
  padding-top: 24px;
}
</style>
