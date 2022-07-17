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

 <!-- Green - I am comfortable with my understanding and pacing of the lesson

Yellow - I am working through my understanding, I would benefit from the teacher slowing down or revisiting the current concept

Red - STOP! I am not understanding and I have a question  -->