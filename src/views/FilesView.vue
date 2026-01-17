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
        :data="fileList"
        :bordered="false"
        :pagination="{ pageSize: 10 }"
        :loading="loading"
        style="margin-top: 24px"
      />
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue'
import { NButton, NIcon, NTag, useMessage, type UploadFileInfo } from 'naive-ui'
import { CloudArrowUp20Regular, DocumentBulletList24Regular, Eye20Regular, Delete20Regular } from '@vicons/fluent'
import { open } from '@tauri-apps/api/dialog'
import { useFileStore } from '@/stores/fileStore'
import { usePersonStore } from '@/stores/personStore'

// 文件记录数据结构（与后端一致）
interface FileRecord {
  id: number
  person_id: number | null
  file_name: string
  file_path: string
  file_type: string
  created_at: string
}

const message = useMessage()
const fileStore = useFileStore()
const personStore = usePersonStore()

// 从 store 获取数据
const files = computed(() => fileStore.files)
const loading = computed(() => fileStore.loading)
const persons = computed(() => personStore.persons)

// 文件列表（添加人员名称）
const fileList = computed(() => {
  return files.value.map(file => ({
    ...file,
    personName: file.person_id
      ? persons.value.find(p => p.id === file.person_id)?.name || '-'
      : '-'
  }))
})

// 表格列配置
const columns = [
  {
    title: '文件名',
    key: 'file_name'
  },
  {
    title: '文件类型',
    key: 'file_type',
    render(row: FileRecord & { personName: string }) {
      return h(NTag, {
        type: getFileTypeTag(row.file_type)
      }, { default: () => row.file_type })
    }
  },
  {
    title: '关联人员',
    key: 'personName',
    render(row: FileRecord & { personName: string }) {
      return row.personName || '-'
    }
  },
  {
    title: '创建时间',
    key: 'created_at'
  },
  {
    title: '操作',
    key: 'actions',
    render(row: FileRecord) {
      return h('div', { style: { display: 'flex', gap: '8px' } }, [
        h(NButton, {
          size: 'small',
          quaternary: true,
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

// 页面加载时获取数据
onMounted(async () => {
  await Promise.all([
    fileStore.fetchFiles(),
    personStore.fetchPersons()
  ])
})

// 点击上传 - 打开文件选择对话框
const handleUpload = async () => {
  try {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: 'Documents',
          extensions: ['pdf', 'doc', 'docx', 'xls', 'xlsx', 'png', 'jpg', 'jpeg'],
        },
      ],
    })

    if (selected && Array.isArray(selected)) {
      for (const filePath of selected) {
        const fileName = filePath.split(/[/\\]/).pop() || ''
        const ext = fileName.split('.').pop()?.toLowerCase() || ''
        const fileType = getFileType(ext)
        await fileStore.addFile(null, fileName, filePath, fileType)
      }
      message.success(`成功添加 ${selected.length} 个文件`)
    }
  } catch (e) {
    message.error('选择文件失败')
    console.error(e)
  }
}

// 拖拽处理
const handleFileChange = (options: { file: UploadFileInfo }) => {
  const file = options.file
  if (file.file) {
    message.info(`已选择文件: ${file.name}`)
  }
}

// 查看文件
const handleView = (row: FileRecord) => {
  message.info(`查看文件: ${row.file_name}`)
}

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

// 根据扩展名获取文件类型
const getFileType = (ext: string): string => {
  const types: Record<string, string> = {
    pdf: 'PDF',
    doc: 'Word',
    docx: 'Word',
    xls: 'Excel',
    xlsx: 'Excel',
    png: 'Image',
    jpg: 'Image',
    jpeg: 'Image',
  }
  return types[ext] || 'Unknown'
}

// 删除文件
const handleDelete = async (id: number) => {
  try {
    await fileStore.deleteFile(id)
    message.success('删除成功')
  } catch {
    message.error('删除失败')
  }
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
