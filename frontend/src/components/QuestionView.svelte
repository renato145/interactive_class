<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";
  import { identity } from "svelte/internal";
  import DivTimer from "./DivTimer.svelte";

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
    <div class="text-sm">
      <button class="btn">Edit</button>
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
  <div class="mt-4 flex flex-wrap gap-4">
    {#each question.options as option, i}
      <div
        class="cursor-default rounded-lg bg-slate-300 px-4 py-2 text-lg shadow"
      >
        {option}
        {question.answers[i]}
      </div>
    {/each}
  </div>
  <div class="mt-4 text-lg font-bold">
    <p>Answers: <span>{answers}/{connections}</span></p>
  </div>
</div>
