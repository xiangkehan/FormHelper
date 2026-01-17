# FormHelper 项目规划

## 项目概述

一个桌面端表格识别应用，用于识别 PDF、图片、Word、Excel 文件中的表格内容，并将数据记录到 SQLite 数据库中。

## 技术栈

| 层级 | 技术选型 |
|------|----------|
| 前端框架 | Vue 3 + Vite |
| UI 组件库 | Naive UI 或 Element Plus |
| 桌面框架 | Tauri 2.0 |
| 后端语言 | Rust |
| 数据库 | SQLite |
| 表格识别 | Rust OCR 库（tesseract）或 Python 桥接 |

## 核心功能

### 1. 文件处理
- [ ] 支持 PDF 文件解析（提取表格）
- [ ] 支持图片格式（PNG、JPG）OCR 识别
- [ ] 支持 Word 文档表格提取
- [ ] 支持 Excel 文件读取
- [ ] 拖拽上传文件

### 2. 表格识别
- [ ] 本地 OCR 引擎集成
- [ ] 表格结构检测
- [ ] 单元格内容提取
- [ ] 可配置 OCR 语言包
- [ ] 云服务 API 扩展接口

### 3. 数据管理
- [ ] SQLite 数据库存储
- [ ] 以人员为主键的数据记录
- [ ] 数据增删改查
- [ ] 数据导出功能（Excel/CSV）

### 4. i18n 多语言
- [ ] 简体中文
- [ ] English
- [ ] 语言切换功能

## 数据库设计

### persons 表（人员主表）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键，自增 |
| name | TEXT | 姓名 |
| created_at | DATETIME | 创建时间 |
| updated_at | DATETIME | 更新时间 |

### files 表（文件记录）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键，自增 |
| person_id | INTEGER | 关联人员ID |
| file_name | TEXT | 文件名 |
| file_path | TEXT | 文件存储路径 |
| file_type | TEXT | 文件类型 |
| created_at | DATETIME | 创建时间 |

### table_records 表（表格数据）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键，自增 |
| file_id | INTEGER | 关联文件ID |
| person_id | INTEGER | 关联人员ID |
| content | TEXT | 表格内容（JSON存储） |
| created_at | DATETIME | 创建时间 |

## 项目结构

```
FormHelper/
├── src/                    # Vue 前端源码
│   ├── assets/            # 静态资源
│   ├── components/        # 公共组件
│   ├── views/            # 页面视图
│   ├── stores/           # Pinia 状态管理
│   ├── locales/          # i18n 语言文件
│   ├── App.vue
│   └── main.ts
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/     # Tauri 命令
│   │   ├── db/           # 数据库操作
│   │   └── ocr/          # OCR 处理
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── README.md
```

## 开发阶段规划

### 第一阶段：基础架构
- [ ] 初始化 Tauri + Vue 项目
- [ ] 配置 i18n（中文/英文）
- [ ] 创建数据库和基础 CRUD 接口

### 第二阶段：文件处理
- [ ] 实现文件选择和预览
- [ ] 集成 PDF 解析
- [ ] 集成图片 OCR（tesseract）
- [ ] 集成 Word 解析
- [ ] 集成 Excel 解析

### 第三阶段：表格识别
- [ ] 表格区域检测
- [ ] 行列结构识别
- [ ] 数据提取和存储
- [ ] 数据展示和编辑

### 第四阶段：完善与优化
- [ ] 云服务 API 扩展
- [ ] 数据导出功能
- [ ] 界面优化
- [ ] 打包发布
