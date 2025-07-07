<script lang="ts">
  import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte'
  import NewTodoModal from '$lib/components/NewTodoModal.svelte'

  let { children } = $props()
</script>

<NewTodoModal />
<Navbar />

{@render children()}
