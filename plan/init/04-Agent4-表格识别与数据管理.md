# Agent 4 提示词 - 表格识别与数据管理

## 进度管理（开始前必须执行）

```bash
cd progress
python progress_manager.py check phase3_parallel
# 确认依赖满足后再继续
python progress_manager.py start phase3_parallel
```

### 任务对照表（Agent 4）

| 完成后执行 | 任务 |
|-----------|------|
| `python progress_manager.py task phase3_parallel record_rs` | 记录 CRUD 命令 |
| `python progress_manager.py task phase3_parallel export_rs` | 导出命令 |
| `python progress_manager.py task phase3_parallel db_functions` | 数据库查询函数 |
| `python progress_manager.py task phase3_parallel TableEditor_vue` | 表格编辑组件 |
| `python progress_manager.py task phase3_parallel DataView_vue` | 数据管理视图 |
| `python progress_manager.py task phase3_parallel data_working` | 数据功能可用 |

---

## 角色定义

你是一个全栈开发者，负责实现表格识别、数据展示编辑和数据导出功能。

## 任务目标

实现表格结构检测、数据存储管理、前端展示编辑和数据导出功能。

## 工作目录

```
FormHelper/
```

## 当前依赖

- Agent 1: Tauri 命令框架（必须完成）
- Agent 2: 前端界面（必须完成）
- Agent 3: OCR 集成（建议完成）

## 具体任务

### 1. 后端：表格记录命令

在 `src-tauri/src/commands/` 创建 `record.rs`：

```rust
use crate::db::{self, TableRecord};
use crate::commands::person::DbState;
use tauri::State;

/// 获取表格记录
#[tauri::command]
pub fn get_records(
    file_id: Option<i32>,
    person_id: Option<i32>,
    state: State<DbState>,
) -> Result<Vec<TableRecord>, String> {
    let conn = state.get_conn()?;
    match (file_id, person_id) {
        (Some(fid), _) => db::get_table_records_by_file(&conn, fid),
        (_, Some(pid)) => db::get_table_records_by_person(&conn, pid),
        _ => db::get_all_table_records(&conn),
    }
    .map_err(|e| e.to_string())
}

/// 更新表格内容（支持编辑）
#[tauri::command]
pub fn update_record(
    id: i32,
    content: String,
    state: State<DbState>,
) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::update_table_record(&conn, id, &content)?;
    Ok(())
}

/// 删除记录
#[tauri::command]
pub fn delete_record(id: i32, state: State<DbState>) -> Result<(), String> {
    let conn = state.get_conn()?;
    db::delete_table_record(&conn, id)?;
    Ok(())
}
```

### 2. 后端：数据导出命令

在 `src-tauri/src/commands/` 创建 `export.rs`：

```rust
use crate::commands::person::DbState;
use tauri::State;

/// 导出人员数据到 Excel
#[tauri::command]
pub fn export_to_excel(
    person_id: i32,
    file_path: String,
    state: State<DbState>,
) -> Result<(), String> {
    let conn = state.get_conn()?;

    // 获取人员的所有记录
    let records = crate::db::get_table_records_by_person(&conn, person_id)
        .map_err(|e| e.to_string())?;

    // TODO: 使用 calamine 创建 Excel 文件
    // 创建工作簿，写入数据，保存文件

    Ok(())
}

/// 导出人员数据到 CSV
#[tauri::command]
pub fn export_to_csv(
    person_id: i32,
    file_path: String,
    state: State<DbState>,
) -> Result<(), String> {
    let conn = state.get_conn()?;

    // 获取人员的所有记录
    let records = crate::db::get_table_records_by_person(&conn, person_id)
        .map_err(|e| e.to_string())?;

    // 生成 CSV 格式
    let mut csv = String::new();
    for record in records {
        csv.push_str(&record.content);
        csv.push('\n');
    }

    // 写入文件（处理 UTF-8 BOM）
    std::fs::write(&file_path, csv)?;

    Ok(())
}
```

### 3. 更新 db/mod.rs

添加表格记录查询函数：

```rust
/// 获取文件的所有表格记录
pub fn get_table_records_by_file(
    conn: &Connection,
    file_id: i32,
) -> Result<Vec<TableRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, file_id, person_id, content, created_at FROM table_records WHERE file_id = ?1",
    )?;
    // 查询并返回
    Ok(Vec::new())
}

/// 获取人员的所有表格记录
pub fn get_table_records_by_person(
    conn: &Connection,
    person_id: i32,
) -> Result<Vec<TableRecord>> {
    // 查询并返回
    Ok(Vec::new())
}

/// 获取所有表格记录
pub fn get_all_table_records(conn: &Connection) -> Result<Vec<TableRecord>> {
    // 查询并返回
    Ok(Vec::new())
}

/// 更新表格记录
pub fn update_table_record(conn: &Connection, id: i32, content: &str) -> Result<()> {
    conn.execute(
        "UPDATE table_records SET content = ?1 WHERE id = ?2",
        [content, &id.to_string()],
    )?;
    Ok(())
}

/// 删除表格记录
pub fn delete_table_record(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM table_records WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}
```

### 4. 更新 main.rs

添加新命令：

```rust
use commands::record::{get_records, update_record, delete_record};
use commands::export::{export_to_excel, export_to_csv};

invoke_handler![
    greet,
    // 人员命令
    get_persons,
    create_person,
    update_person,
    delete_person,
    // 文件命令
    get_files,
    add_file,
    delete_file,
    // 处理命令
    process_file,
    // 记录命令
    get_records,
    update_record,
    delete_record,
    // 导出命令
    export_to_excel,
    export_to_csv,
]
```

### 5. 前端：表格编辑组件

在 `src/components/` 创建 `TableEditor.vue`：

```vue
<template>
  <div class="table-editor">
    <n-data-table
      :columns="columns"
      :data="tableData"
      :bordered="true"
      :single-line="false"
      :max-height="400"
      :scroll-x="800"
    />

    <div class="actions" style="margin-top: 16px">
      <n-button type="primary" @click="handleSave" :loading="saving">
        保存
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { NInput, useMessage } from 'naive-ui'

const props = defineProps<{
  recordId: number
  content: TableContent
}>()

const emit = defineEmits(['saved'])

const message = useMessage()
const saving = ref(false)

// 表格数据
const tableData = computed(() => props.content.rows)

// 动态列定义
const columns = computed(() => {
  const cols: any[] = [
    {
      title: '#',
      key: 'rowIndex',
      width: 60,
      render: (row: any, index: number) => h('span', null, `${index + 1}`)
    }
  ]

  // 表头列
  props.content.headers.forEach((header, colIndex) => {
    cols.push({
      title: header,
      key: `col_${colIndex}`,
      width: 120,
      render: (row: any, rowIndex: number) => {
        return h(NInput, {
          value: row[colIndex] || '',
          onUpdateValue: (val: string) => {
            tableData.value[rowIndex][colIndex] = val
          }
        })
      }
    })
  })

  return cols
})

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

interface TableContent {
  headers: string[]
  rows: string[][]
}
</script>
```

### 6. 前端：数据管理视图

在 `src/views/` 创建 `DataView.vue`：

```vue
<parameter name="content"><template>
  <div class="data-view">
    <!-- 人员选择 -->
    <n-card :bordered="false" style="margin-bottom: 16px">
      <n-space>
        <n-select
          v-model:value="selectedPerson"
          :options="personOptions"
          placeholder="选择人员"
          style="width: 200px"
          clearable
        />
        <n-button @click="loadData" :loading="loading">刷新</n-button>
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
      />
    </n-card>

    <!-- 表格预览/编辑弹窗 -->
    <n-modal v-model:show="showEditor" preset="card" style="width: 80%" title="表格数据">
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
import { ref, computed, onMounted } from 'vue'
import { NButton, NIcon, useMessage, type DataTableColumns } from 'naive-ui'
import { Eye20Regular, Trash20Regular } from '@vicons/fluent'
import { invoke } from '@tauri-apps/api/tauri'
import TableEditor from '@/components/TableEditor.vue'

interface TableRecord {
  id: number
  file_id: number
  person_id: number | null
  content: string
  created_at: string
}

interface TableContent {
  headers: string[]
  rows: string[][]
}

const message = useMessage()
const loading = ref(false)
const records = ref<TableRecord[]>([])
const selectedPerson = ref<number | null>(null)
const showEditor = ref(false)
const selectedRecord = ref<TableRecord | null>(null)

// 人员选项
const personOptions = computed(() => {
  // TODO: 从 personStore 获取
  return []
})

// 表格列定义
const columns: DataTableColumns<TableRecord> = [
  { title: 'ID', key: 'id', width: 60 },
  { title: '文件ID', key: 'file_id', width: 80 },
  { title: '创建时间', key: 'created_at' },
  {
    title: '操作',
    key: 'actions',
    render: (row) => h('div', { style: { display: 'flex', gap: '8px' } }, [
      h(NButton, {
        size: 'small',
        onClick: () => viewRecord(row)
      }, {
        icon: () => h(NIcon, null, { default: () => h(Eye20Regular) })
      }),
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

// 导出
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

onMounted(() => {
  loadData()
})
</script>
```

### 7. 添加 i18n 文本

更新 `src/locales/zh-CN.json`：

```json
{
  "data": {
    "title": "数据管理",
    "selectPerson": "选择人员",
    "exportExcel": "导出 Excel",
    "exportCsv": "导出 CSV",
    "view": "查看",
    "export": "导出",
    "noData": "暂无数据"
  }
}
```

更新 `src/locales/en-US.json`：

```json
{
  "data": {
    "title": "Data Management",
    "selectPerson": "Select Person",
    "exportExcel": "Export Excel",
    "exportCsv": "Export CSV",
    "view": "View",
    "export": "Export",
    "noData": "No data"
  }
}
```

### 8. 添加路由

更新 `src/router/index.ts`：

```typescript
const routes: RouteRecordRaw[] = [
  // ... 现有路由
  {
    path: '/data',
    name: 'data',
    component: () => import('@/views/DataView.vue')
  }
]
```

## 模块结构

```
src-tauri/src/
├── commands/
│   ├── mod.rs              # 更新：添加 record, export
│   ├── record.rs           # 新增：记录 CRUD
│   └── export.rs           # 新增：导出功能
└── db/
    └── mod.rs              # 更新：添加查询函数

src/
├── components/
│   └── TableEditor.vue     # 新增：表格编辑组件
└── views/
    └── DataView.vue        # 新增：数据管理视图
```

## 产出要求

- [ ] `record.rs` 表格记录命令
- [ ] `export.rs` 数据导出命令
- [ ] `db/mod.rs` 查询函数更新
- [ ] `TableEditor.vue` 表格编辑组件
- [ ] `DataView.vue` 数据管理视图
- [ ] i18n 文本更新

## 注意事项

1. 每个文件都要有简洁清晰的中文注释
2. 表格数据使用 JSON 存储
3. 导出功能处理编码问题
4. 前端编辑支持实时保存

## 依赖检查

执行前请确认：

- [ ] Agent 1 已完成（Tauri 命令框架）
- [ ] Agent 2 已完成（前端界面）
- [ ] Agent 3 已完成（OCR 集成）

如果依赖不满足，请停止并告知用户。

## 后续步骤

Agent 4 完成后：

1. 合并所有 Agent 的 worktree 到 main
2. 测试完整功能流程
3. 修复发现的问题
4. 可选：实现云服务 API 扩展（第五阶段）
