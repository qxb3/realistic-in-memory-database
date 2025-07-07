<script lang="ts">
  import { isNewTodoModalOpen } from '../stores/todos'
</script>

<nav class="px-8 py-4 border border-b border-black/20">
  <div class="container max-w-5xl mx-auto flex justify-between items-center">
    <!-- left -->
    <div>
      <h1 class="text-2xl font-bold">ToDon't</h1>
    </div>

    <!-- right -->
    <div class="flex justify-between items-center gap-4">
      <button
        on:click={() => isNewTodoModalOpen.set(true)}
        class="bg-blue-500 text-white px-4 py-2 rounded-xl hover:bg-blue-700 hover:shadow-xl cursor-pointer transition-all duration-500"
      >
        New +
      </button>
    </div>
  </div>
</nav>
