<template>
  <div class="data-view">
    <!-- 顶部操作栏 -->
    <n-card :bordered="false" style="margin-bottom: 16px">
      <n-space>
        <!-- 人员选择器 -->
        <n-select
          v-model:value="selectedPerson"
          :options="personOptions"
          :placeholder="$t('data.selectPerson')"
          style="width: 200px"
          clearable
          @update:value="loadData"
        />
        <!-- 刷新按钮 -->
        <n-button @click="loadData" :loading="loading">
          {{ $t('common.search') }}
        </n-button>
        <!-- 导出按钮组 -->
        <n-button @click="handleExport('excel')">导出 Excel</n-button>
        <n-button @click="handleExport('csv')">导出 CSV</n-button>
      </n-space>
    </n-card>

    <!-- 记录列表 -->
    <n-card :bordered="false">
      <n-data-table
        :columns="columns"
        :data="records"
        :loading="loading"
        :pagination="{ pageSize: 10 }"
        :bordered="true"
      />
    </n-card>

    <!-- 表格预览/编辑弹窗 -->
    <n-modal v-model:show="showEditor" preset="card" style="width: 80%" :title="$t('data.view')">
      <TableEditor
        v-if="selectedRecord"
        :record-id="selectedRecord.id"
        :content="selectedRecord.content"
        @saved="loadData"
      />
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue'
import { NButton, NIcon, useMessage, type DataTableColumns, NTag } from 'naive-ui'
import { Eye20Regular, Trash20Regular } from '@vicons/fluent'
import { invoke } from '@tauri-apps/api/tauri'
import TableEditor from '@/components/TableEditor.vue'

// 表格记录接口
interface TableRecord {
  id: number
  file_id: number
  person_id: number | null
  content: string
  created_at: string
}

// 表格内容接口
interface TableContent {
  headers: string[]
  rows: string[][]
}

// 解析内容
function parseContent(content: string): TableContent {
  try {
    return JSON.parse(content)
  } catch {
    return { headers: [], rows: [] }
  }
}

const message = useMessage()
const loading = ref(false)
const records = ref<TableRecord[]>([])
const selectedPerson = ref<number | null>(null)
const showEditor = ref(false)
const selectedRecord = ref<TableRecord | null>(null)

// 人员选项（模拟数据，实际应从 store 获取）
const personOptions = computed(() => {
  return records.value
    .filter(r => r.person_id !== null)
    .map(r => ({
      label: `人员 ${r.person_id}`,
      value: r.person_id
    }))
    .filter((v, i, a) => a.findIndex(t => t.value === v.value) === i)
})

// 表格列定义
const columns: DataTableColumns<TableRecord> = [
  { title: 'ID', key: 'id', width: 60 },
  { title: '文件ID', key: 'file_id', width: 80 },
  {
    title: '表格尺寸',
    key: 'size',
    width: 120,
    render: (row) => {
      const content = parseContent(row.content)
      const size = `${content.headers.length}列 x ${content.rows.length}行`
      return h(NTag, { size: 'small', type: 'info' }, { default: () => size })
    }
  },
  { title: '创建时间', key: 'created_at' },
  {
    title: '操作',
    key: 'actions',
    width: 120,
    render: (row) => h('div', { style: { display: 'flex', gap: '8px' } }, [
      // 查看/编辑按钮
      h(NButton, {
        size: 'small',
        onClick: () => viewRecord(row)
      }, {
        icon: () => h(NIcon, null, { default: () => h(Eye20Regular) })
      }),
      // 删除按钮
      h(NButton, {
        size: 'small',
        type: 'error',
        onClick: () => deleteRecord(row.id)
      }, {
        icon: () => h(NIcon, null, { default: () => h(Trash20Regular) })
      })
    ])
  }
]

// 加载数据
const loadData = async () => {
  loading.value = true
  try {
    records.value = await invoke('get_records', {
      fileId: null,
      personId: selectedPerson.value
    })
  } catch (e) {
    message.error('加载数据失败')
  } finally {
    loading.value = false
  }
}

// 查看记录
const viewRecord = (record: TableRecord) => {
  selectedRecord.value = record
  showEditor.value = true
}

// 删除记录
const deleteRecord = async (id: number) => {
  try {
    await invoke('delete_record', { id })
    message.success('删除成功')
    loadData()
  } catch {
    message.error('删除失败')
  }
}

// 导出处理
const handleExport = async (format: 'excel' | 'csv') => {
  if (!selectedPerson.value) {
    message.warning('请先选择人员')
    return
  }

  try {
    const { invoke } = await import('@tauri-apps/api/tauri')
    await invoke('export_to_' + format, {
      personId: selectedPerson.value,
      filePath: `export_${Date.now()}.${format}`
    })
    message.success('导出成功')
  } catch {
    message.error('导出失败')
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadData()
})
</script>

<style scoped>
.data-view {
  padding: 16px;
}
</style>
