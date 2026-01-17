# FormHelper 项目进度管理

## 目录结构

```
progress/
├── progress.json        # 进度数据（JSON格式）
├── progress_manager.py  # 进度管理工具（Python）
├── progress.bat         # Windows 批处理版本
└── README.md            # 本文件
```

## 使用方法

### 查看当前进度

**Python:**
```bash
cd progress
python progress_manager.py read
```

**Windows:**
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
   任务: 0/5

⏳ Agent 2: 前后端对接
   状态: pending
   ✗ 依赖未满足 (phase1 未完成)
   任务: 0/6

...
```

### 开始某阶段

```bash
python progress_manager.py start phase1
```

### 完成某阶段

```bash
python progress_manager.py complete phase1
```

### 完成单个任务

```bash
python progress_manager.py task phase1 commands_person_rs
```

### 检查依赖

```bash
python progress_manager.py check phase2
```

### 添加备注

```bash
python progress_manager.py note "发现一个 bug，需要修复"
```

## 阶段顺序

1. **phase1** - Agent 1: 完善 Tauri 命令
2. **phase2** - Agent 2: 前后端对接
3. **phase3_parallel** - Agent 3 & 4: OCR 与表格识别（并行）

## Agent 执行规则

1. **开始前必须检查依赖** - 使用 `check` 命令
2. **开始时更新状态** - 使用 `start` 命令
3. **完成后更新状态** - 使用 `complete` 命令
4. **重要发现添加备注** - 使用 `note` 命令
