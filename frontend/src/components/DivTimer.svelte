<script lang="ts">
  import { fade } from "svelte/transition";
  import { onDestroy } from "svelte";
  import { questionsStore } from "../stores/ws";

  let klass = "";
  export { klass as class };
  export let question_id: string;

  let time_secs,
    remaining = 0,
    timer = null;

  let unsubscribe = questionsStore.subscribe((question) => {
    if (question !== null) {
      if (question_id === question.id) {
        if (timer !== null) clearInterval(timer);
        time_secs = 5;
        remaining = 5;
        timer = setInterval(() => {
          remaining = Math.max(0, remaining - 1);
        }, 1000);
      }
    }
  });

  onDestroy(() => unsubscribe());
</script>

{#if remaining > 0}
  <div transition:fade class={`flex items-center ${klass}`}>
    <div class="h-4 w-full bg-gray-400">
      <div
        class="h-full bg-green-600"
        style:width={`${(100 * remaining) / time_secs}%`}
      />
    </div>
    <p class="px-2 pb-1 text-xl font-bold">{remaining}s</p>
  </div>
{/if}
