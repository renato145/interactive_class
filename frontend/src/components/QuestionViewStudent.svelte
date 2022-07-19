<script lang="ts">
  import type { QuestionPublication } from "bindings/QuestionPublication";

  export let question: QuestionPublication;
</script>

<div class="rounded-lg border border-gray-500 px-8 py-4 shadow">
  <p class="text-3xl font-medium">{question.title}</p>
  <div class="mt-4 flex flex-wrap gap-4">
    {#each question.options as option, i}
      <div
        class="cursor-default rounded-lg bg-slate-300 px-4 py-2 text-lg shadow"
      >
        {option}
      </div>
    {/each}
  </div>
</div>
