# Agent 2 提示词 - 前后端对接

## 进度管理（开始前必须执行）

```bash
cd progress
python progress_manager.py check phase2
# 确认依赖满足后再继续
python progress_manager.py start phase2
```

### 任务对照表

| 完成后执行 | 任务 |
|-----------|------|
| `python progress_manager.py task phase2 personStore_ts` | 创建 personStore.ts |
| `python progress_manager.py task phase2 fileStore_ts` | 创建 fileStore.ts |
| `python progress_manager.py task phase2 PersonsView_vue` | PersonsView 对接 |
| `python progress_manager.py task phase2 FilesView_vue` | FilesView 对接 |
| `python progress_manager.py task phase2 HomeView_vue` | HomeView 对接 |
| `python progress_manager.py task phase2 api_working` | API 测试通过 |
| `python progress_manager.py complete phase2` | 完成阶段2 |

---

## 角色定义

你是一个前端开发者，负责实现 FormHelper 的前后端数据对接。

## 任务目标

**注意：前端界面已部分完成。你的任务是将模拟数据替换为真实 Tauri 命令调用。**

## 工作目录

```
FormHelper/src/
```

## 当前完成情况

### 已完成的界面

```
src/
├── views/
│   ├── HomeView.vue         # 首页（最近文件展示）
│   ├── PersonsView.vue      # 人员管理（增删改查）
│   ├── FilesView.vue        # 文件处理（拖拽上传）
│   └── SettingsView.vue     # 设置页面
├── stores/
│   └── index.ts             # appStore（主题、语言）
└── locales/
    ├── zh-CN.json           # 中文
    └── en-US.json           # 英文
```

### 前端视图使用模拟数据

- `PersonsView.vue`: `persons` 数组使用模拟数据
- `FilesView.vue`: `files` 数组使用模拟数据
- `HomeView.vue`: `recentFiles` 使用模拟数据

### 需要的 Tauri 命令（Agent 1 实现）

```rust
// 人员命令
get_persons() -> Vec<Person>
create_person(name: String) -> i32
update_person(id: i32, name: String) -> ()
delete_person(id: i32) -> ()

// 文件命令
get_files() -> Vec<FileRecord>
add_file(person_id: Option<i32>, file_name, file_path, file_type) -> i32
delete_file(id: i32) -> ()
```

## 具体任务

### 1. 创建人员状态管理

在 `src/stores/` 创建 `personStore.ts`：

```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface Person {
  id: number
  name: string
  created_at: string
  updated_at: string
}

export const usePersonStore = defineStore('person', () => {
  const persons = ref<Person[]>([])
  const loading = ref(false)

  // 获取所有人员
  const fetchPersons = async () => {
    loading.value = true
    try {
      persons.value = await invoke('get_persons')
    } catch (e) {
      console.error('获取人员列表失败:', e)
    } finally {
      loading.value = false
    }
  }

  // 创建人员
  const addPerson = async (name: string) => {
    try {
      const id = await invoke('create_person', { name })
      await fetchPersons()
      return id
    } catch (e) {
      console.error('创建人员失败:', e)
      throw e
    }
  }

  // 更新人员
  const updatePerson = async (id: number, name: string) => {
    try {
      await invoke('update_person', { id, name })
      await fetchPersons()
    } catch (e) {
      console.error('更新人员失败:', e)
      throw e
    }
  }

  // 删除人员
  const deletePerson = async (id: number) => {
    try {
      await invoke('delete_person', { id })
      persons.value = persons.value.filter(p => p.id !== id)
    } catch (e) {
      console.error('删除人员失败:', e)
      throw e
    }
  }

  return {
    persons,
    loading,
    fetchPersons,
    addPerson,
    updatePerson,
    deletePerson,
  }
})
```

### 2. 创建文件状态管理

在 `src/stores/` 创建 `fileStore.ts`：

```typescript
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface FileRecord {
  id: number
  person_id: number | null
  file_name: string
  file_path: string
  file_type: string
  created_at: string
}

export const useFileStore = defineStore('file', () => {
  const files = ref<FileRecord[]>([])
  const loading = ref(false)

  // 获取所有文件
  const fetchFiles = async () => {
    loading.value = true
    try {
      files.value = await invoke('get_files')
    } catch (e) {
      console.error('获取文件列表失败:', e)
    } finally {
      loading.value = false
    }
  }

  // 添加文件记录
  const addFile = async (
    personId: number | null,
    fileName: string,
    filePath: string,
    fileType: string
  ) => {
    try {
      const id = await invoke('add_file', {
        personId,
        fileName,
        filePath,
        fileType,
      })
      await fetchFiles()
      return id
    } catch (e) {
      console.error('添加文件失败:', e)
      throw e
    }
  }

  // 删除文件
  const deleteFile = async (id: number) => {
    try {
      await invoke('delete_file', { id })
      files.value = files.value.filter(f => f.id !== id)
    } catch (e) {
      console.error('删除文件失败:', e)
      throw e
    }
  }

  return {
    files,
    loading,
    fetchFiles,
    addFile,
    deleteFile,
  }
})
```

### 3. 更新 PersonsView.vue

替换模拟数据为真实 API 调用：

```typescript
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NButton, NIcon, NTag, useMessage } from 'naive-ui'
import { Add20Regular, Edit20Regular, Trash20Regular } from '@vicons/fluent'
import { usePersonStore } from '@/stores/personStore'

const message = useMessage()
const personStore = usePersonStore()

const searchKeyword = ref('')
const showAddModal = ref(false)
const isEditing = ref(false)
const editingId = ref<number | null>(null)

const formData = ref({ name: '' })

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

// 页面加载时获取数据
onMounted(() => {
  personStore.fetchPersons()
})

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

// 删除人员
const handleDelete = async (id: number) => {
  try {
    await personStore.deletePerson(id)
    message.success('删除成功')
  } catch {
    message.error('删除失败')
  }
}

// ... 其他代码保持不变
</script>
```

### 4. 更新 FilesView.vue

替换模拟数据并实现文件选择：

```typescript
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NButton, NIcon, NTag, useMessage, type UploadFileInfo } from 'naive-ui'
import { CloudArrowUp20Regular, DocumentBulletList24Regular, Eye20Regular, Delete20Regular } from '@vicons/fluent'
import { open } from '@tauri-apps/api/dialog'
import { useFileStore } from '@/stores/fileStore'

const message = useMessage()
const fileStore = useFileStore()

// 页面加载时获取数据
onMounted(() => {
  fileStore.fetchFiles()
})

// 文件列表
const files = computed(() => fileStore.files)

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
        const ext = fileName.split('.').pop()?.toLowerCase()
        const fileType = getFileType(ext || '')

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
const handleFileChange = async (options: { file: UploadFileInfo }) => {
  const file = options.file
  if (file.file) {
    // 本地文件处理
    message.info(`已选择文件: ${file.name}`)
  }
}

// 获取文件类型标签
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

// ... 其他代码保持不变
</script>
```

### 5. 更新 HomeView.vue

显示真实最近文件：

```typescript
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

onMounted(() => {
  fileStore.fetchFiles()
})

const goToFiles = () => {
  router.push({ name: 'files' })
}
</script>
```

### 6. 添加 store 到 main.ts

```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { usePersonStore } from '@/stores/personStore'
import { useFileStore } from '@/stores/fileStore'

const app = createApp(App)

// 初始化 store
const pinia = createPinia()
app.use(pinia)

// 预加载数据
usePersonStore().fetchPersons()
useFileStore().fetchFiles()

app.mount('#app')
```

## 产出要求

- [ ] `src/stores/personStore.ts` 人员状态管理
- [ ] `src/stores/fileStore.ts` 文件状态管理
- [ ] `PersonsView.vue` 对接真实 API
- [ ] `FilesView.vue` 对接真实 API + 文件选择
- [ ] `HomeView.vue` 显示真实数据

## 注意事项

1. 每个文件都要有简洁清晰的中文注释
2. 使用 `computed` 保持响应式
3. 在 `onMounted` 中初始化数据
4. 错误处理要显示友好的提示
5. 完成后可以并行启动 Agent 3 和 Agent 4

## 依赖检查

执行前请确认：
- [ ] Agent 1 已完成（Tauri 命令已实现）
- [ ] `invoke` API 可用
- [ ] 前端开发环境可运行

如果依赖不满足，请停止并告知用户。
