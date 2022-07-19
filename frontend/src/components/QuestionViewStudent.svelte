<script lang="ts">
  import type { QuestionPublication } from "bindings/QuestionPublication";

  export let question: QuestionPublication, answerQuestion: (i: number) => void;

  let selection = -1;

  const submitAnswerQuestion = (i: number) => {
    selection = i;
    answerQuestion(selection);
  };
</script>

<div class="rounded-lg border border-gray-500 px-8 py-4 shadow">
  <p class="text-3xl font-medium">{question.title}</p>
  <div class="mt-4 flex flex-wrap gap-4">
    {#each question.options as option, i}
      <button
        class={`btn ${i === selection ? "bg-green-400" : ""}`}
        on:click={() => submitAnswerQuestion(i)}
      >
        {option}
      </button>
    {/each}
  </div>
</div>
