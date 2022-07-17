<script lang="ts">
  import { fade } from "svelte/transition";
  export let error_msg;
</script>

{#if error_msg}
  <div transition:fade class="rounded bg-red-300 px-3 py-2 text-lg font-medium">
    {error_msg}
  </div>
{/if}
