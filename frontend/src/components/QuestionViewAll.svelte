<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";
  import QuestionView from "./QuestionView.svelte";

  export let questions: QuestionInfo[],
    publishQuestion: (question_id: string) => void;
</script>

{#if questions}
  <div class="flex gap-8">
    {#each questions as question}
      <QuestionView {question} {publishQuestion} />
    {/each}
  </div>
{/if}
