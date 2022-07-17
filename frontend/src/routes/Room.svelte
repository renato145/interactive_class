<script lang="ts">
  import WsStatus from "../components/WSStatus.svelte";
  import { getWSStore } from "../stores/ws";
  export let roomName;

  let wsStore = getWSStore(roomName, "student");
  let color;
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus status={$wsStore.status} class="self-center" />
    <p class="ml-3 text-3xl">Room: {roomName}</p>
  </div>
  <div class="mt-8">
    <svg
      class={`mx-auto aspect-square w-1/2 max-w-md ${
        color === "green"
          ? "fill-green-500"
          : color === "yellow"
          ? "fill-yellow-300"
          : color === "red"
          ? "fill-red-500"
          : "fill-gray-300"
      }`}
      viewBox="0 0 218 227"
    >
      <path d="M175 1H42L1 226H217L175 1Z" />
    </svg>
  </div>

  <div class="mt-4">
    <div class="mx-auto flex w-1/2 max-w-md flex-col space-y-2">
      <button
        class="rounded bg-green-200 p-2 text-left shadow hover:bg-green-400"
        on:click={() => {
          color = "green";
        }}
      >
        Green - I am comfortable with my understanding and pacing of the lesson
      </button>

      <button
        class="rounded bg-yellow-200 p-2 text-left shadow hover:bg-yellow-300"
        on:click={() => {
          color = "yellow";
        }}
      >
        Yellow - I am working through my understanding, I would benefit from the
        teacher slowing down or revisiting the current concept
      </button>

      <button
        class="rounded bg-red-200 p-2 text-left shadow hover:bg-red-400"
        on:click={() => {
          color = "red";
        }}
      >
        Red - STOP! I am not understanding and I have a question
      </button>
    </div>
  </div>
</div>
