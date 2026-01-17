/**
 * 人员状态管理 - 负责与后端 Tauri 命令对接
 */
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// 人员数据结构
interface Person {
  id: number
  name: string
  created_at: string
  updated_at: string
}

export const usePersonStore = defineStore('person', () => {
  const persons = ref<Person[]>([])      // 人员列表
  const loading = ref(false)              // 加载状态

  // 获取所有人员
  const fetchPersons = async () => {
    loading.value = true
    try {
      persons.value = await invoke('get_persons') as Person[]
    } catch (e) {
      console.error('获取人员列表失败:', e)
    } finally {
      loading.value = false
    }
  }

  // 创建人员
  const addPerson = async (name: string) => {
    try {
      const id = await invoke('create_person', { name }) as number
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

  // 计算属性：人员数量
  const personCount = computed(() => persons.value.length)

  return {
    persons,
    loading,
    personCount,
    fetchPersons,
    addPerson,
    updatePerson,
    deletePerson,
  }
})
