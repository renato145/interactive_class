<script lang="ts">
  import { onDestroy } from "svelte";
  import WsStatus from "../components/WSStatus.svelte";
  import { wsMessageStore, sendWSMessage, wsStatusStore } from "../stores/ws";
  export let roomName;

  let connections = 0;
  const unsubscribe1 = wsMessageStore.subscribe((msg) => {
    if (msg !== null) {
      if (msg.kind === "RoomInfo") {
        connections = +msg.payload.connections;
      }
    }
  });

  const unsubscribe2 = wsStatusStore.subscribe((status) => {
    if (status === "started") {
      sendWSMessage({
        task: "RoomConnect",
        payload: roomName,
      });
    }
  });

  onDestroy(() => {
    unsubscribe1();
    unsubscribe2();
  });
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus class="self-center" />
    <p class="ml-3 text-3xl">Room: {roomName} ({connections})</p>
  </div>

  <!-- <div class="mt-4">
    <button class="btn" on:click={send_msg}>Send message</button>
  </div> -->
</div>
