<template>
  <div class="files-container">
    <n-card :bordered="false">
      <template #header>
        <div class="card-header">
          <span>{{ $t('files.title') }}</span>
          <n-button type="primary" @click="handleUpload">
            <template #icon>
              <n-icon><CloudArrowUp20Regular /></n-icon>
            </template>
            {{ $t('files.upload') }}
          </n-button>
        </div>
      </template>

      <!-- 拖拽上传区域 -->
      <n-upload
        ref="uploadRef"
        :show-file-list="false"
        multiple
        directory-dnd
        @change="handleFileChange"
      >
        <n-upload-dragger>
          <div style="margin-bottom: 12px">
            <n-icon size="48" :depth="3">
              <DocumentBulletList24Regular />
            </n-icon>
          </div>
          <n-text style="font-size: 16px">
            {{ $t('files.dragDrop') }}
          </n-text>
        </n-upload-dragger>
      </n-upload>

      <!-- 文件列表 -->
      <n-data-table
        :columns="columns"
        :data="files"
        :bordered="false"
        :pagination="{ pageSize: 10 }"
        style="margin-top: 24px"
      />
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, h } from 'vue'
import { NButton, NIcon, NTag, useMessage, type UploadFileInfo } from 'naive-ui'
import { CloudArrowUp20Regular, DocumentBulletList24Regular, Eye20Regular, Delete20Regular } from '@vicons/fluent'

interface FileRecord {
  id: number
  fileName: string
  fileType: string
  personName: string | null
  status: 'pending' | 'processing' | 'success' | 'error'
  createdAt: string
}

const message = useMessage()
const files = ref<FileRecord[]>([
  {
    id: 1,
    fileName: '员工信息表.pdf',
    fileType: 'PDF',
    personName: '张三',
    status: 'success',
    createdAt: '2024-01-15 10:30'
  },
  {
    id: 2,
    fileName: '成绩单.png',
    fileType: 'Image',
    personName: null,
    status: 'pending',
    createdAt: '2024-01-16 14:20'
  }
])

const columns = [
  {
    title: '文件名',
    key: 'fileName'
  },
  {
    title: '文件类型',
    key: 'fileType',
    render(row: FileRecord) {
      return h(NTag, {
        type: row.fileType === 'PDF' ? 'error' : row.fileType === 'Image' ? 'success' : 'warning'
      }, { default: () => row.fileType })
    }
  },
  {
    title: '关联人员',
    key: 'personName',
    render(row: FileRecord) {
      return row.personName || '-'
    }
  },
  {
    title: '状态',
    key: 'status',
    render(row: FileRecord) {
      const type = row.status === 'success' ? 'success' : row.status === 'error' ? 'error' : 'warning'
      const text = row.status === 'success' ? '已识别' : row.status === 'processing' ? '处理中' : row.status === 'error' ? '失败' : '待处理'
      return h(NTag, { type }, { default: () => text })
    }
  },
  {
    title: '创建时间',
    key: 'createdAt'
  },
  {
    title: '操作',
    key: 'actions',
    render(row: FileRecord) {
      return h('div', { style: { display: 'flex', gap: '8px' } }, [
        h(NButton, {
          size: 'small',
          quaternary: true,
          disabled: row.status !== 'success',
          onClick: () => handleView(row)
        }, {
          icon: () => h(NIcon, null, { default: () => h(Eye20Regular) })
        }),
        h(NButton, {
          size: 'small',
          quaternary: true,
          type: 'error',
          onClick: () => handleDelete(row.id)
        }, {
          icon: () => h(NIcon, null, { default: () => h(Delete20Regular) })
        })
      ])
    }
  }
]

const handleUpload = () => {
  message.info('点击上传功能待实现')
}

const handleFileChange = (options: { file: UploadFileInfo }) => {
  message.info(`已选择文件: ${options.file.name}`)
}

const handleView = (row: FileRecord) => {
  message.info(`查看文件: ${row.fileName}`)
}

const handleDelete = (id: number) => {
  files.value = files.value.filter(f => f.id !== id)
  message.success('删除成功')
}
</script>

<style scoped>
.files-container {
  max-width: 1200px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
