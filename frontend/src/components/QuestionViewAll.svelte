<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";
  import QuestionView from "./QuestionView.svelte";

  export let connections: number,
    questions: QuestionInfo[],
    publishQuestion: (question_id: string, secs: number) => void;
</script>

{#if questions}
  <div class="flex flex-wrap gap-6">
    {#each questions as question}
      <QuestionView {question} {publishQuestion} {connections} />
    {/each}
  </div>
{/if}
