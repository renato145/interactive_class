<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";

  export let connections: number, question: QuestionInfo,
    publishQuestion: (question_id: string) => void;

  $: answers = question.answers.reduce((acc, x) => acc+x);
</script>

<div class="rounded-lg border border-gray-500 px-8 py-4 shadow">
  <p class="text-3xl font-medium">{question.title}</p>
  <div class="mt-2">
    <button class="btn" on:click={() => publishQuestion(question.id)}
      >Publish</button
    >
    <button class="btn">Edit</button>
    <button class="btn">Delete</button>
  </div>
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
