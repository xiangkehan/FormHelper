<template>
  <div class="home-container">
    <n-card :bordered="false" class="welcome-card">
      <n-h1>{{ $t('home.welcome') }}</n-h1>
      <n-p>{{ $t('home.description') }}</n-p>
      <n-button type="primary" size="large" @click="goToFiles">
        {{ $t('home.quickStart') }}
      </n-button>
    </n-card>

    <n-card :title="$t('home.recentFiles')" :bordered="false" class="recent-card">
      <template v-if="recentFiles.length > 0">
        <n-list>
          <n-list-item v-for="file in recentFiles" :key="file.id">
            <n-thing>
              <template #header>
                {{ file.file_name }}
              </template>
              <template #header-extra>
                <n-tag :type="getFileTypeTag(file.file_type)">
                  {{ file.file_type }}
                </n-tag>
              </template>
              <template #description>
                {{ file.created_at }}
              </template>
            </n-thing>
          </n-list-item>
        </n-list>
      </template>
      <template v-else>
        <n-empty :description="$t('home.noRecentFiles')" />
      </template>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useFileStore } from '@/stores/fileStore'

const router = useRouter()
const fileStore = useFileStore()

// 取最近5个文件
const recentFiles = computed(() => {
  return fileStore.files.slice(0, 5)
})

// 页面加载时获取数据
onMounted(() => {
  fileStore.fetchFiles()
})

// 获取文件类型标签颜色
const getFileTypeTag = (type: string): 'default' | 'success' | 'warning' | 'error' => {
  const types: Record<string, 'default' | 'success' | 'warning' | 'error'> = {
    PDF: 'error',
    Image: 'success',
    Word: 'warning',
    Excel: 'info'
  }
  return types[type] || 'default'
}

// 跳转到文件页面
const goToFiles = () => {
  router.push({ name: 'files' })
}
</script>

<style scoped>
.home-container {
  max-width: 800px;
  margin: 0 auto;
}

.welcome-card {
  margin-bottom: 24px;
  text-align: center;
  padding: 40px;
}

.welcome-card h1 {
  margin-bottom: 12px;
}

.welcome-card p {
  color: var(--n-text-color-secondary);
  margin-bottom: 24px;
}
</style>
