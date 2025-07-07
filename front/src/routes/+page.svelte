<script lang="ts">
  import { onMount } from 'svelte'
  import { todos } from '$lib/stores/todos'

  onMount(() => {
    setInterval(async () => {
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
    }, 1000)
  })
</script>

<div class="container max-w-5xl mx-auto p-8">
  <div>
    <h1 class="text-4xl">ToDon'ts</h1>
  </div>

  <div class="space-y-8 min-h-96">
    {#if $todos.length <= 0}
      <div class="py-8">
        <p class="text-black/40 text-center text-xl">Empty tasks</p>
      </div>
    {/if}

    {#each $todos as todo}
      <div class="flex items-center gap-8 p-4 border border-black/20 rounded-xl mt-12">
        <button
          on:click={() => {}}
          class="p-7 border border-black/40 rounded-xl hover:bg-black/20 cursor-pointer transition-colors duration-500"
        >
          ✓
        </button>

        <div>
          <h1 class="text-2xl">* {todo.task}</h1>
          <p class="text-black/40 text-xl">{todo.description}</p>
        </div>
      </div>
    {/each}
  </div>
</div>
