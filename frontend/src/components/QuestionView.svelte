<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";
  import DivTimer from "./DivTimer.svelte";
  import { questionsStore } from "../stores/ws";

  export let connections: number,
    question: QuestionInfo,
    publishQuestion: (question_id: string) => void;

  $: answers = question.answers.reduce((acc, x) => acc + x);

  const doPublishQuestion = () => {
    publishQuestion(question.id);
    questionsStore.set({
      id: question.id,
      title: question.title,
      options: question.options,
    });
  };
</script>

<div class="rounded-lg border border-gray-500 px-8 pt-4 shadow">
  <p class="text-3xl font-medium">{question.title}</p>
  <DivTimer class="-mx-4" question_id={question.id} />
  <div class="mt-2">
    <button class="btn" on:click={doPublishQuestion}>Publish</button>
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
