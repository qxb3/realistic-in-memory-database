<script lang="ts">
  import { isNewTodoModalOpen, todos } from '../stores/todos'

  let task = ''
  let description = ''

  $: if (!$isNewTodoModalOpen) {
    task = ''
    description = ''
  }

  async function saveTodo() {
    const saveResp = await fetch('http://localhost:4321/db', {
      method: 'CREATE',
      headers: {
        'Content-Type': 'application/json',
        'Data': JSON.stringify({ task, description, isDone: false })
      }
    })

    if (saveResp.status !== 200) {
      console.error('Failed to add json')
      return
    }

    const listResp = await fetch('http://localhost:4321/db', {
      method: 'LIST',
      headers: {
        'Content-Type': 'application/json',
      }
    })

    const list = (await listResp.json())
      .map((v: { id: number, data: string }) => {
        let data = JSON.parse(v.data)

        return ({
          id: v.id,
          task: data.task,
          description: data.description,
          isDone: data.isDone
        })
      })

    todos.set(list)

    isNewTodoModalOpen.set(false)
  }
</script>

<div
  class:hidden={!$isNewTodoModalOpen}
  class:grid={$isNewTodoModalOpen}
  class="hidden place-items-center fixed w-screen h-screen bg-black/80 transition-all duration-500"
>
  <div class="min-w-xl max-w-xl bg-white rounded-xl">
    <div class="border-b border-black/20 p-4">
      <h1 class="text-2xl font-bold">New Todo</h1>
    </div>

    <div class="p-4 space-y-8">
      <div>
        <p class="text-lg">Task</p>
        <input
          bind:value={task}
          class="p-2 rounded-xl border border-black/20 w-full"
          type="text"
          placeholder="New Task"
        />
      </div>

      <div>
        <p class="text-lg">Description</p>
        <input
          bind:value={description}
          class="p-2 rounded-xl border border-black/20 w-full"
          type="text"
          placeholder="Task Description"
        />
      </div>
    </div>

    <div class="p-4 flex justify-end items-center gap-4 border-t border-black/20">
      <button
        on:click={() => isNewTodoModalOpen.set(false)}
        class="bg-red-500 text-white px-4 py-2 rounded-xl hover:bg-red-700 hover:shadow-xl cursor-pointer transition-all duration-500"
      >
        Cancel
      </button>

      <button
        on:click={saveTodo}
        class="bg-blue-500 text-white px-4 py-2 rounded-xl hover:bg-blue-700 hover:shadow-xl cursor-pointer transition-all duration-500"
      >
        Save
      </button>
    </div>
  </div>
</div>
