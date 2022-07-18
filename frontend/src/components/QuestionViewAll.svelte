<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";
  import QuestionView from "./QuestionView.svelte";

  export let questions: QuestionInfo;
</script>

{#if questions}
  <div class="flex gap-8">
    {#each Object.entries(questions) as [id, question]}
      <QuestionView questionId={id} {question} />
    {/each}
  </div>
{/if}
