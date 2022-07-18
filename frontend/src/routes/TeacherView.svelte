<script lang="ts">
  import WsError from "../components/WSError.svelte";
  import WsStatus from "../components/WSStatus.svelte";
  import CupBlock from "../components/CupBlock.svelte";
  import { getWSStore } from "../stores/ws";
  export let roomName;

  let { wsStore, createQuestion } = getWSStore(roomName, "Teacher");
  $: unanswered =
    $wsStore.connections -
    $wsStore.cups.green -
    $wsStore.cups.yellow -
    $wsStore.cups.red;
  $: greenCups = $wsStore.cups.green / $wsStore.connections;
  $: yellowCups = $wsStore.cups.yellow / $wsStore.connections;
  $: redCups = $wsStore.cups.red / $wsStore.connections;
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
    <WsError error_msg={$wsStore.error_msg} />
  </div>
  <div class="ml-8 mt-8">
    <div class="w-[300px] h-[400px] rounded shadow ring ring-gray-500">
      <CupBlock color="Red" cupPerc={redCups} cups={$wsStore.cups.red} />
      <CupBlock
        color="Yellow"
        cupPerc={yellowCups}
        cups={$wsStore.cups.yellow}
      />
      <CupBlock color="Green" cupPerc={greenCups} cups={$wsStore.cups.green} />
      <CupBlock cupPerc={unanswered / $wsStore.connections} cups={unanswered} />
    </div>
  </div>
</div>
