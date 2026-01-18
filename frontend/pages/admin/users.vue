<template>
  <NuxtLayout>
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ $t('pages.admin.users.title') }}
          </h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {{ $t('pages.admin.users.description') }}
          </p>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="flex items-center justify-center py-12">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>

      <!-- Users Table -->
      <div v-else class="relative overflow-x-auto shadow-md sm:rounded-lg">
        <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
          <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
            <tr>
              <th scope="col" class="px-6 py-3">{{ $t('pages.admin.users.userName') }}</th>
              <th scope="col" class="px-6 py-3">{{ $t('pages.admin.users.userEmail') }}</th>
              <th scope="col" class="px-6 py-3">{{ $t('pages.admin.users.userRoles') }}</th>
              <th scope="col" class="px-6 py-3">{{ $t('common.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="userData in users"
              :key="userData.user.id"
              class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
            >
              <td class="px-6 py-4 font-medium text-gray-900 dark:text-white">
                {{ userData.user.user_fullname }}
              </td>
              <td class="px-6 py-4">
                {{ userData.user.user_email }}
              </td>
              <td class="px-6 py-4">
                <div class="flex flex-wrap gap-2">
                  <span
                    v-for="role in userData.roles"
                    :key="role.id"
                    :class="[
                      'px-2 py-1 text-xs font-semibold rounded',
                      role.is_admin
                        ? 'text-blue-800 bg-blue-100 dark:bg-blue-900 dark:text-blue-300'
                        : 'text-green-800 bg-green-100 dark:bg-green-900 dark:text-green-300'
                    ]"
                  >
                    {{ role.name }}
                    <button
                      @click="removeRoleFromUser(userData.user.id, role.id)"
                      class="ml-1 text-xs hover:text-red-600"
                    >
                      ×
                    </button>
                  </span>
                  <span
                    v-if="userData.roles.length === 0"
                    class="px-2 py-1 text-xs font-semibold text-gray-500 bg-gray-100 rounded dark:bg-gray-700 dark:text-gray-400"
                  >
                    {{ $t('pages.admin.users.noRoles') }}
                  </span>
                </div>
              </td>
              <td class="px-6 py-4">
                <button
                  @click="openAssignRoleModal(userData)"
                  class="font-medium text-blue-600 dark:text-blue-500 hover:underline"
                >
                  {{ $t('pages.admin.users.assignRole') }}
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- No Users Message -->
      <div
        v-if="!loading && users.length === 0"
        class="p-6 text-center text-gray-500 dark:text-gray-400"
      >
        {{ $t('pages.admin.users.noUsers') }}
      </div>
    </div>

    <!-- Assign Role Modal -->
    <div
      v-if="showAssignModal"
      class="fixed inset-0 z-50 flex items-center justify-center overflow-y-auto bg-gray-900 bg-opacity-50"
      @click.self="closeAssignModal"
    >
      <div class="relative w-full max-w-md p-4">
        <div class="relative bg-white rounded-lg shadow dark:bg-gray-800">
          <div class="flex items-center justify-between p-5 border-b rounded-t dark:border-gray-600">
            <h3 class="text-xl font-medium text-gray-900 dark:text-white">
              {{ $t('pages.admin.users.assignRole') }}
            </h3>
            <button
              @click="closeAssignModal"
              class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white"
            >
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
              </svg>
            </button>
          </div>
          <div class="p-6 space-y-6">
            <div>
              <p class="mb-4 text-sm text-gray-700 dark:text-gray-300">
                {{ $t('pages.admin.users.userName') }}: <strong>{{ selectedUser?.user.user_fullname }}</strong>
              </p>
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {{ $t('pages.admin.users.selectRole') }}
              </label>
              <select
                v-model="selectedRoleId"
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white"
              >
                <option value="">{{ $t('pages.admin.users.selectRole') }}</option>
                <option
                  v-for="role in availableRoles"
                  :key="role.role.id"
                  :value="role.role.id"
                >
                  {{ role.role.name }}
                </option>
              </select>
            </div>
          </div>
          <div class="flex items-center p-6 space-x-2 border-t border-gray-200 rounded-b dark:border-gray-600">
            <button
              @click="assignRole"
              :disabled="!selectedRoleId"
              class="text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ $t('pages.admin.users.assignRole') }}
            </button>
            <button
              @click="closeAssignModal"
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

interface Role {
  id: string
  name: string
  description: string | null
  is_admin: boolean
}

interface User {
  id: string
  sub: string
  user_email: string
  user_fullname: string
  organization: string | null
  group_id: string | null
}

interface UserWithRoles {
  user: User
  roles: Role[]
}

interface RoleWithPermissions {
  role: Role
  permissions: any[]
}

const users = ref<UserWithRoles[]>([])
const availableRoles = ref<RoleWithPermissions[]>([])
const loading = ref(true)
const showAssignModal = ref(false)
const selectedUser = ref<UserWithRoles | null>(null)
const selectedRoleId = ref('')

onMounted(async () => {
  await loadData()
})

async function loadData() {
  loading.value = true
  try {
    const [usersData, rolesData] = await Promise.all([
      get<UserWithRoles[]>('/admin/users', 'users'),
      get<RoleWithPermissions[]>('/admin/roles', 'roles')
    ])
    users.value = usersData
    availableRoles.value = rolesData
  } catch (error) {
    console.error('Failed to load data:', error)
  } finally {
    loading.value = false
  }
}

function openAssignRoleModal(userData: UserWithRoles) {
  selectedUser.value = userData
  selectedRoleId.value = ''
  showAssignModal.value = true
}

function closeAssignModal() {
  showAssignModal.value = false
  selectedUser.value = null
  selectedRoleId.value = ''
}

async function assignRole() {
  if (!selectedUser.value || !selectedRoleId.value) return

  try {
    await post(`/admin/users/${selectedUser.value.user.id}/roles`, {
      role_id: selectedRoleId.value
    })
    closeAssignModal()
    await loadData()
  } catch (error) {
    console.error('Failed to assign role:', error)
    alert('Failed to assign role')
  }
}

async function removeRoleFromUser(userId: string, roleId: string) {
  if (!confirm('Are you sure you want to remove this role?')) {
    return
  }

  try {
    await post(`/admin/users/${userId}/roles/remove`, {
      role_id: roleId
    })
    await loadData()
  } catch (error) {
    console.error('Failed to remove role:', error)
    alert('Failed to remove role')
  }
}
</script>
