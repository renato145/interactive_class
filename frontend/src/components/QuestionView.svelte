<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";

  export let question: QuestionInfo;
</script>

<div class="px-8 py-4 rounded-lg shadow border border-gray-500">
  <p class="text-3xl font-medium">{question.title}</p>
  <div class="flex flex-wrap gap-4 mt-4">
    {#each question.options as option, i}
      <div
        class="px-4 py-2 text-lg bg-slate-300 rounded-lg shadow cursor-default"
      >
        {option}
        {question.answers[i]}
      </div>
    {/each}
  </div>
</div>
