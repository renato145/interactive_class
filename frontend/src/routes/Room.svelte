<script lang="ts">
  import WsStatus from "../components/WSStatus.svelte";
  import { getWSStore } from "../stores/ws";
  export let roomName;

  let wsStore = getWSStore(roomName);
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus status={$wsStore.status} class="self-center" />
    <p class="ml-3 text-3xl">Room: {roomName} ({$wsStore.connections})</p>
  </div>
</div>
