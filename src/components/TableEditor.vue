<template>
  <div class="table-editor">
    <!-- 表格展示与编辑 -->
    <n-data-table
      :columns="columns"
      :data="tableData"
      :bordered="true"
      :single-line="false"
      :max-height="400"
      :scroll-x="scrollWidth"
    />
    <!-- 操作按钮 -->
    <div class="actions" style="margin-top: 16px">
      <n-button type="primary" @click="handleSave" :loading="saving">
        {{ $t('common.save') }}
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { NInput, NButton, useMessage } from 'naive-ui'
import { invoke } from '@tauri-apps/api/tauri'

// 定义属性
const props = defineProps<{
  recordId: number      // 记录ID
  content: TableContent // 表格内容
}>()

// 定义事件
const emit = defineEmits(['saved'])

const message = useMessage()
const saving = ref(false)

// 计算滚动宽度
const scrollWidth = computed(() => {
  return props.content.headers.length * 120 + 100
})

// 表格数据（响应式）
const tableData = ref<any[]>([...props.content.rows])

// 动态列定义
const columns = computed(() => {
  const cols: any[] = [
    {
      title: '#',           // 行号列
      key: 'rowIndex',
      width: 60,
      render: (_row: any, index: number) => h('span', null, `${index + 1}`)
    }
  ]

  // 表头列
  props.content.headers.forEach((header, colIndex) => {
    cols.push({
      title: header,        // 列标题
      key: `col_${colIndex}`,
      width: 120,
      render: (row: any, rowIndex: number) => {
        return h(NInput, {
          value: row[colIndex] || '',                    // 当前单元格值
          placeholder: '',
          onUpdateValue: (val: string) => {
            tableData.value[rowIndex][colIndex] = val    // 更新数据
          }
        })
      }
    })
  })

  return cols
})

// 保存处理
const handleSave = async () => {
  saving.value = true
  try {
    await invoke('update_record', {
      id: props.recordId,
      content: JSON.stringify({
        headers: props.content.headers,
        rows: tableData.value
      })
    })
    message.success('保存成功')
    emit('saved')
  } catch (e) {
    message.error('保存失败')
  } finally {
    saving.value = false
  }
}

// 表格内容接口
interface TableContent {
  headers: string[]
  rows: string[][]
}
</script>

<style scoped>
.table-editor {
  padding: 16px;
}
.actions {
  display: flex;
  justify-content: flex-end;
}
</style>
