<script lang="ts">
  import type { QuestionPublication } from "bindings/QuestionPublication";
  import DivTimer from "./DivTimer.svelte";

  export let question: QuestionPublication, answerQuestion: (i: number) => void;

  let selection = -1;

  const submitAnswerQuestion = (i: number) => {
    selection = i;
    answerQuestion(selection);
  };
</script>

<div class="min-w-[300px] rounded-lg border border-gray-500 px-8 py-4 shadow">
  <p class="text-3xl font-medium">{question.title}</p>
  <DivTimer class="-mx-4" question_id={question.id} />
  <div class="mt-4 flex flex-wrap gap-4">
    {#each question.options as option, i}
      <button
        class={`btn min-w-[100px] ${
          i === selection ? "bg-green-600 hover:bg-green-400" : ""
        }`}
        on:click={() => submitAnswerQuestion(i)}
      >
        {option}
      </button>
    {/each}
  </div>
</div>
