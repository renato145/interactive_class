<script lang="ts">
  import { onDestroy } from "svelte";
  import { wsMessageStore, sendWSMessage } from "../stores/ws";
  export let roomName;

  const unsubscribe = wsMessageStore.subscribe((msg) => {
    console.log("received msg:", msg);
  });

  const send_msg = () => {
    sendWSMessage({
      task: "RoomConnect",
      payload: "roomName"
    });
  };

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  Some room: {roomName}

  <button class="btn" on:click={send_msg}>Send message</button>
</div>
