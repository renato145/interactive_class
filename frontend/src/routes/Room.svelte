<script lang="ts">
  import { onDestroy } from "svelte";
  import { wsMessageStore } from "../stores/ws";
  export let roomName;

  const unsubscribe = wsMessageStore.subscribe((msg) => {
    console.log("received msg:", msg);
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  Some room: {roomName}
</div>
