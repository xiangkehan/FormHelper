# FormHelper 项目进度管理

## 目录结构

```
progress/
├── progress.json   # 进度数据（JSON格式）
├── progress.bat    # Windows 批处理版本
└── README.md       # 本文件
```

## 使用方法

所有命令在 `progress/` 目录下执行：

```batch
cd progress
```

### 查看当前进度

```batch
progress.bat read
```

**输出示例:**

```

============================================================
FormHelper 项目进度
最后更新: 2026-01-18T00:35:00+08:00
============================================================

⏳ Agent 1: 完善 Tauri 命令
   状态: pending
   依赖满足
   任务: 0/6

⏳ Agent 2: 前后端对接
   状态: pending
   ✗ 依赖未满足 (phase1 未完成)
   任务: 0/7

...
```

### 开始某阶段

```batch
progress.bat start phase1
```

### 完成某阶段

```batch
progress.bat complete phase1
```

### 完成单个任务

```batch
progress.bat task phase1 commands_person_rs
```

### 检查依赖

```batch
progress.bat check phase2
```

### 添加备注

```batch
progress.bat note 发现一个 bug
```

## 阶段顺序

1. **phase1** - Agent 1: 完善 Tauri 命令
2. **phase2** - Agent 2: 前后端对接
3. **phase3_parallel** - Agent 3 & 4: OCR 与表格识别（并行）

## Agent 执行规则

1. **开始前必须检查依赖** - 使用 `progress.bat check <阶段>`
2. **开始时更新状态** - 使用 `progress.bat start <阶段>`
3. **完成后更新状态** - 使用 `progress.bat complete <阶段>`
4. **重要发现添加备注** - 使用 `progress.bat note "<消息>"`
