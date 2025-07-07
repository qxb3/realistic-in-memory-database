import { writable } from 'svelte/store'

import type { Todo } from '../../app.d.ts'

export const isNewTodoModalOpen = writable(false)

export const todos = writable<Todo[]>([])
