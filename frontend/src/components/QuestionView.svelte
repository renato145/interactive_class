<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";
  import DivTimer from "./DivTimer.svelte";
  import AnswerBarChart from "./AnswerBarChart.svelte";

  export let connections: number,
    question: QuestionInfo,
    publishQuestion: (question_id: string, secs: number) => void,
    deleteQuestion: (question_id) => void;

  $: answers = question.answers.reduce((acc, x) => acc + x);
  let publishTime = 60;
</script>

<div class="rounded-lg border border-gray-500 px-8 py-4 shadow">
  <div class="flex flex-wrap items-center justify-between">
    <p class="text-3xl font-medium">{question.title}</p>
    <div class="ml-2 text-sm">
      <button class="btn" disabled>Edit</button>
      <button class="btn-cancel" on:click={() => deleteQuestion(question.id)}
        >Delete</button
      >
    </div>
  </div>
  <div class="mt-4 flex">
    <label
      >Publish time (seconds):
      <input class="ml-2 w-16 p-1" type="number" bind:value={publishTime} />
    </label>
    <button
      class="btn ml-4 font-bold"
      on:click={() => publishQuestion(question.id, publishTime)}>Publish</button
    >
  </div>
  <DivTimer class="-mx-4" question_id={question.id} />
  <div class="mt-4">
    <AnswerBarChart {question} />
  </div>
  {#if connections > 0}
    <div class="mt-6 text-2xl font-bold">
      <p>Answers: <span>{answers}/{connections}</span></p>
    </div>
  {/if}
</div>
