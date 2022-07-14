<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import WsStatus from "../components/WSStatus.svelte";
  import { wsMessageStore, sendWSMessage } from "../stores/ws";
  export let roomName;

  let connections = 0;
  const unsubscribe = wsMessageStore.subscribe((msg) => {
    if (msg !== null) {
      if (msg.kind === "RoomInfo") {
        connections = +msg.payload.connections;
      }
    }
  });

  const send_msg = () => {
    sendWSMessage({
      task: "RoomConnect",
      payload: roomName,
    });
  };

  onMount(() => {
    sendWSMessage({
      task: "RoomConnect",
      payload: roomName,
    });
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus class="self-center" />
    <p class="ml-3 text-3xl">Room: {roomName} ({connections})</p>
  </div>

  <div class="mt-4">
    <button class="btn" on:click={send_msg}>Send message</button>
  </div>
</div>
