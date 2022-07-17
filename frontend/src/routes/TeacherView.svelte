<script lang="ts">
  import WsStatus from "../components/WSStatus.svelte";
  import { getWSStore } from "../stores/ws";
  export let roomName;

  let wsStore = getWSStore(roomName, "Teacher");
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus status={$wsStore.status} class="self-center" />
    <p class="ml-3 text-3xl">
      Room: {roomName} ({$wsStore.connections === 0
        ? "no students"
        : $wsStore.connections === 1
        ? "1 student"
        : `${$wsStore.connections} students`}) -
      <a href={`/room/${roomName}`} target="_black" rel="noopener"
        >[student view]</a
      >
    </p>
  </div>
</div>
