<script lang="ts">
  import WsStatus from "../components/WSStatus.svelte";
  import { getWSStore } from "../stores/ws";
  export let roomName;

  let { wsStore } = getWSStore(roomName, "Teacher");
  $: unanswered =
    $wsStore.connections -
    $wsStore.cups.green -
    $wsStore.cups.yellow -
    $wsStore.cups.red;
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
  <div class="mt-4">
    <p>Green: {$wsStore.cups.green}</p>
    <p>Yellow: {$wsStore.cups.yellow}</p>
    <p>Red: {$wsStore.cups.red}</p>
    <p>Unanswered: {unanswered}</p>
  </div>
</div>
