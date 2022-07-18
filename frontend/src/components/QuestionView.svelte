<script lang="ts">
  import type { QuestionState } from "bindings/QuestionState";

  export let questionId: string, question: QuestionState;
</script>

<div class="px-8 py-4 rounded-lg shadow border border-gray-500">
  <p class="text-3xl font-medium">{question.title}</p>
  <div class="flex flex-wrap gap-4 mt-4">
    {#each question.options as d}
      <div
        class="px-4 py-2 text-lg bg-slate-300 rounded-lg shadow cursor-default"
      >
        {d}
      </div>
    {/each}
  </div>
</div>
