<template>
  <NuxtLayout>
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ $t('pages.admin.roles.title') }}
          </h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {{ $t('pages.admin.roles.description') }}
          </p>
        </div>
        <button
          @click="openCreateModal"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        >
          {{ $t('pages.admin.roles.createRole') }}
        </button>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="flex items-center justify-center py-12">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>

      <!-- Roles List -->
      <div v-else class="grid gap-4">
        <div
          v-for="roleData in roles"
          :key="roleData.role.id"
          class="p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700"
        >
          <div class="flex items-start justify-between mb-4">
            <div class="flex-1">
              <div class="flex items-center gap-2">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
                  {{ roleData.role.name }}
                </h2>
                <span
                  v-if="roleData.role.is_admin"
                  class="px-2 py-1 text-xs font-semibold text-blue-800 bg-blue-100 rounded dark:bg-blue-900 dark:text-blue-300"
                >
                  {{ $t('pages.admin.roles.isAdmin') }}
                </span>
              </div>
              <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                {{ roleData.role.description }}
              </p>
            </div>
            <div class="flex gap-2">
              <button
                @click="openEditModal(roleData)"
                class="p-2 text-blue-600 hover:text-blue-700 dark:text-blue-400"
              >
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                </svg>
              </button>
              <button
                v-if="!roleData.role.is_admin"
                @click="deleteRole(roleData.role.id)"
                class="p-2 text-red-600 hover:text-red-700 dark:text-red-400"
              >
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                </svg>
              </button>
            </div>
          </div>

          <!-- Permission Matrix -->
          <div class="mt-4 overflow-x-auto">
            <h3 class="mb-2 text-lg font-semibold text-gray-900 dark:text-white">
              {{ $t('pages.admin.roles.permissionMatrix') }}
            </h3>
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
              <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                  <th scope="col" class="px-4 py-3">{{ $t('pages.admin.roles.page') }}</th>
                  <th scope="col" class="px-2 py-3 text-center">{{ $t('pages.admin.roles.canView') }}</th>
                  <th scope="col" class="px-2 py-3 text-center">{{ $t('pages.admin.roles.canEdit') }}</th>
                  <th scope="col" class="px-2 py-3 text-center">{{ $t('pages.admin.roles.canViewOwn') }}</th>
                  <th scope="col" class="px-2 py-3 text-center">{{ $t('pages.admin.roles.canEditOwn') }}</th>
                  <th scope="col" class="px-2 py-3 text-center">{{ $t('pages.admin.roles.canViewOurs') }}</th>
                  <th scope="col" class="px-2 py-3 text-center">{{ $t('pages.admin.roles.canEditOurs') }}</th>
                  <th scope="col" class="px-4 py-3">{{ $t('common.actions') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="page in availablePages"
                  :key="page"
                  class="bg-white border-b dark:bg-gray-800 dark:border-gray-700"
                >
                  <td class="px-4 py-3 font-medium text-gray-900 dark:text-white">
                    {{ $t(`pages.admin.roles.pages.${page}`) }}
                  </td>
                  <td class="px-2 py-3 text-center">
                    <input
                      type="checkbox"
                      :checked="getPermission(roleData.permissions, page, 'can_view')"
                      @change="updatePermission(roleData.role.id, page, 'can_view', $event.target.checked)"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                  </td>
                  <td class="px-2 py-3 text-center">
                    <input
                      type="checkbox"
                      :checked="getPermission(roleData.permissions, page, 'can_edit')"
                      @change="updatePermission(roleData.role.id, page, 'can_edit', $event.target.checked)"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                  </td>
                  <td class="px-2 py-3 text-center">
                    <input
                      type="checkbox"
                      :checked="getPermission(roleData.permissions, page, 'can_view_own')"
                      @change="updatePermission(roleData.role.id, page, 'can_view_own', $event.target.checked)"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                  </td>
                  <td class="px-2 py-3 text-center">
                    <input
                      type="checkbox"
                      :checked="getPermission(roleData.permissions, page, 'can_edit_own')"
                      @change="updatePermission(roleData.role.id, page, 'can_edit_own', $event.target.checked)"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                  </td>
                  <td class="px-2 py-3 text-center">
                    <input
                      type="checkbox"
                      :checked="getPermission(roleData.permissions, page, 'can_view_ours')"
                      @change="updatePermission(roleData.role.id, page, 'can_view_ours', $event.target.checked)"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                  </td>
                  <td class="px-2 py-3 text-center">
                    <input
                      type="checkbox"
                      :checked="getPermission(roleData.permissions, page, 'can_edit_ours')"
                      @change="updatePermission(roleData.role.id, page, 'can_edit_ours', $event.target.checked)"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                  </td>
                  <td class="px-4 py-3">
                    <button
                      @click="savePermissions(roleData.role.id, page, roleData.permissions)"
                      class="px-3 py-1 text-xs font-medium text-white bg-green-600 rounded hover:bg-green-700 focus:ring-4 focus:ring-green-300"
                    >
                      {{ $t('common.save') }}
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Edit Role Modal -->
    <div
      v-if="showModal"
      class="fixed inset-0 z-50 flex items-center justify-center overflow-y-auto bg-gray-900 bg-opacity-50"
      @click.self="closeModal"
    >
      <div class="relative w-full max-w-md p-4">
        <div class="relative bg-white rounded-lg shadow dark:bg-gray-800">
          <div class="flex items-center justify-between p-5 border-b rounded-t dark:border-gray-600">
            <h3 class="text-xl font-medium text-gray-900 dark:text-white">
              {{ modalMode === 'create' ? $t('pages.admin.roles.createRole') : $t('pages.admin.roles.editRole') }}
            </h3>
            <button
              @click="closeModal"
              class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white"
            >
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
              </svg>
            </button>
          </div>
          <div class="p-6 space-y-6">
            <div>
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {{ $t('pages.admin.roles.roleName') }}
              </label>
              <input
                v-model="modalData.name"
                type="text"
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white"
                required
              />
            </div>
            <div>
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {{ $t('pages.admin.roles.roleDescription') }}
              </label>
              <textarea
                v-model="modalData.description"
                rows="3"
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white"
              ></textarea>
            </div>
            <div class="flex items-center">
              <input
                v-model="modalData.is_admin"
                type="checkbox"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
              />
              <label class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                {{ $t('pages.admin.roles.isAdmin') }}
              </label>
            </div>
          </div>
          <div class="flex items-center p-6 space-x-2 border-t border-gray-200 rounded-b dark:border-gray-600">
            <button
              @click="saveRole"
              class="text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            >
              {{ $t('common.save') }}
            </button>
            <button
              @click="closeModal"
              class="text-gray-500 bg-white hover:bg-gray-100 focus:ring-4 focus:ring-gray-300 rounded-lg border border-gray-200 text-sm font-medium px-5 py-2.5 hover:text-gray-900 focus:z-10 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-500 dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-gray-600"
            >
              {{ $t('common.cancel') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const { get, post } = useApi()

interface Permission {
  id: string
  role_id: string
  page: string
  can_view: boolean
  can_edit: boolean
  can_view_own: boolean
  can_edit_own: boolean
  can_view_ours: boolean
  can_edit_ours: boolean
}

interface Role {
  id: string
  name: string
  description: string | null
  is_admin: boolean
}

interface RoleWithPermissions {
  role: Role
  permissions: Permission[]
}

const roles = ref<RoleWithPermissions[]>([])
const loading = ref(true)
const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const modalData = ref({
  id: '',
  name: '',
  description: '',
  is_admin: false
})

const availablePages = ['dashboard', 'users', 'roles', 'profile', 'preferences', 'support']

const permissionState = ref<Record<string, Record<string, Record<string, boolean>>>>({})

onMounted(async () => {
  await loadRoles()
})

async function loadRoles() {
  loading.value = true
  try {
    roles.value = await get<RoleWithPermissions[]>('/admin/roles', 'roles')
    
    // Initialize permission state
    roles.value.forEach(roleData => {
      if (!permissionState.value[roleData.role.id]) {
        permissionState.value[roleData.role.id] = {}
      }
      availablePages.forEach(page => {
        const perm = roleData.permissions.find(p => p.page === page)
        permissionState.value[roleData.role.id][page] = {
          can_view: perm?.can_view || false,
          can_edit: perm?.can_edit || false,
          can_view_own: perm?.can_view_own || false,
          can_edit_own: perm?.can_edit_own || false,
          can_view_ours: perm?.can_view_ours || false,
          can_edit_ours: perm?.can_edit_ours || false
        }
      })
    })
  } catch (error) {
    console.error('Failed to load roles:', error)
  } finally {
    loading.value = false
  }
}

function getPermission(permissions: Permission[], page: string, field: string): boolean {
  if (!permissionState.value[permissions[0]?.role_id]) return false
  if (!permissionState.value[permissions[0]?.role_id][page]) return false
  return permissionState.value[permissions[0]?.role_id][page][field] || false
}

function updatePermission(roleId: string, page: string, field: string, value: boolean) {
  if (!permissionState.value[roleId]) {
    permissionState.value[roleId] = {}
  }
  if (!permissionState.value[roleId][page]) {
    permissionState.value[roleId][page] = {
      can_view: false,
      can_edit: false,
      can_view_own: false,
      can_edit_own: false,
      can_view_ours: false,
      can_edit_ours: false
    }
  }
  permissionState.value[roleId][page][field] = value
}

async function savePermissions(roleId: string, page: string, permissions: Permission[]) {
  try {
    const permData = permissionState.value[roleId][page]
    await post(`/admin/roles/${roleId}/permissions`, {
      page,
      ...permData
    })
    alert('Permissions updated successfully')
    await loadRoles()
  } catch (error) {
    console.error('Failed to save permissions:', error)
    alert('Failed to save permissions')
  }
}

function openCreateModal() {
  modalMode.value = 'create'
  modalData.value = {
    id: '',
    name: '',
    description: '',
    is_admin: false
  }
  showModal.value = true
}

function openEditModal(roleData: RoleWithPermissions) {
  modalMode.value = 'edit'
  modalData.value = {
    id: roleData.role.id,
    name: roleData.role.name,
    description: roleData.role.description || '',
    is_admin: roleData.role.is_admin
  }
  showModal.value = true
}

function closeModal() {
  showModal.value = false
}

async function saveRole() {
  try {
    if (modalMode.value === 'create') {
      await post('/admin/roles', {
        name: modalData.value.name,
        description: modalData.value.description || null,
        is_admin: modalData.value.is_admin
      })
    } else {
      await post(`/admin/roles/${modalData.value.id}`, {
        name: modalData.value.name,
        description: modalData.value.description || null,
        is_admin: modalData.value.is_admin
      })
    }
    closeModal()
    await loadRoles()
  } catch (error) {
    console.error('Failed to save role:', error)
    alert('Failed to save role')
  }
}

async function deleteRole(roleId: string) {
  if (!confirm('Are you sure you want to delete this role?')) {
    return
  }
  
  try {
    await post(`/admin/roles/${roleId}/delete`, {})
    await loadRoles()
  } catch (error) {
    console.error('Failed to delete role:', error)
    alert('Failed to delete role')
  }
}
</script>
