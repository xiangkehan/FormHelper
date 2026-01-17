<template>
  <div class="persons-container">
    <n-card :bordered="false">
      <template #header>
        <div class="card-header">
          <span>{{ $t('persons.title') }}</span>
          <n-button type="primary" @click="showAddModal = true">
            <template #icon>
              <n-icon><Add20Regular /></n-icon>
            </template>
            {{ $t('persons.addPerson') }}
          </n-button>
        </div>
      </template>

      <n-input
        v-model:value="searchKeyword"
        :placeholder="$t('persons.searchPlaceholder')"
        clearable
        style="margin-bottom: 16px"
      />

      <n-data-table
        :columns="columns"
        :data="filteredPersons"
        :bordered="false"
        :pagination="{ pageSize: 10 }"
        :loading="loading"
      />
    </n-card>

    <!-- 添加/编辑弹窗 -->
    <n-modal v-model:show="showAddModal" preset="dialog" :title="isEditing ? $t('common.edit') : $t('persons.addPerson')">
      <n-form :model="formData" label-placement="left" label-width="80px">
        <n-form-item :label="$t('persons.name')" required>
          <n-input v-model:value="formData.name" />
        </n-form-item>
      </n-form>
      <template #action>
        <n-button @click="showAddModal = false">{{ $t('common.cancel') }}</n-button>
        <n-button type="primary" @click="handleSave">{{ $t('common.save') }}</n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue'
import { NButton, NIcon, NTag, useMessage } from 'naive-ui'
import { Add20Regular, Edit20Regular, Trash20Regular } from '@vicons/fluent'
import { usePersonStore } from '@/stores/personStore'

// 人员数据结构（与后端一致）
interface Person {
  id: number
  name: string
  created_at: string
  updated_at: string
}

const message = useMessage()
const personStore = usePersonStore()

const searchKeyword = ref('')
const showAddModal = ref(false)
const isEditing = ref(false)
const editingId = ref<number | null>(null)

const formData = ref({
  name: ''
})

// 从 store 获取数据
const persons = computed(() => personStore.persons)
const loading = computed(() => personStore.loading)

// 过滤人员
const filteredPersons = computed(() => {
  if (!searchKeyword.value) return persons.value
  return persons.value.filter(p =>
    p.name.toLowerCase().includes(searchKeyword.value.toLowerCase())
  )
})

// 表格列配置
const columns = [
  {
    title: 'ID',
    key: 'id',
    width: 80
  },
  {
    title: '姓名',
    key: 'name'
  },
  {
    title: '创建时间',
    key: 'created_at'
  },
  {
    title: '操作',
    key: 'actions',
    render(row: Person) {
      return h('div', { style: { display: 'flex', gap: '8px' } }, [
        h(NButton, {
          size: 'small',
          quaternary: true,
          onClick: () => handleEdit(row)
        }, {
          icon: () => h(NIcon, null, { default: () => h(Edit20Regular) })
        }),
        h(NButton, {
          size: 'small',
          quaternary: true,
          type: 'error',
          onClick: () => handleDelete(row.id)
        }, {
          icon: () => h(NIcon, null, { default: () => h(Trash20Regular) })
        })
      ])
    }
  }
]

// 页面加载时获取数据
onMounted(() => {
  personStore.fetchPersons()
})

// 编辑人员
const handleEdit = (row: Person) => {
  isEditing.value = true
  editingId.value = row.id
  formData.value.name = row.name
  showAddModal.value = true
}

// 删除人员
const handleDelete = async (id: number) => {
  try {
    await personStore.deletePerson(id)
    message.success('删除成功')
  } catch {
    message.error('删除失败')
  }
}

// 保存人员
const handleSave = async () => {
  if (!formData.value.name) {
    message.error('请输入姓名')
    return
  }

  try {
    if (isEditing.value && editingId.value) {
      await personStore.updatePerson(editingId.value, formData.value.name)
    } else {
      await personStore.addPerson(formData.value.name)
    }
    showAddModal.value = false
    formData.value.name = ''
    isEditing.value = false
    editingId.value = null
    message.success('保存成功')
  } catch {
    message.error('保存失败')
  }
}
</script>

<style scoped>
.persons-container {
  max-width: 1000px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
