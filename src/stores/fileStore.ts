/**
 * 文件状态管理 - 负责与后端 Tauri 命令对接
 */
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// 文件记录数据结构
interface FileRecord {
  id: number
  person_id: number | null
  file_name: string
  file_path: string
  file_type: string
  created_at: string
}

export const useFileStore = defineStore('file', () => {
  const files = ref<FileRecord[]>([])   // 文件列表
  const loading = ref(false)             // 加载状态

  // 获取所有文件
  const fetchFiles = async () => {
    loading.value = true
    try {
      files.value = await invoke('get_files') as FileRecord[]
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
      }) as number
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

  // 计算属性：文件数量
  const fileCount = computed(() => files.value.length)

  return {
    files,
    loading,
    fileCount,
    fetchFiles,
    addFile,
    deleteFile,
  }
})
