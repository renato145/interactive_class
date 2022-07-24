<script lang="ts">
  import type { QuestionInfo } from "bindings/QuestionInfo";

  export let question: QuestionInfo;
  $: max_n = question.answers.reduce((acc, x) => Math.max(acc, x));
  $: data = question.options.map((option, i) => ({
    option,
    n: question.answers[i],
    n_perc: (100 * question.answers[i]) / max_n,
  }));
</script>

<div class="flex flex-col">
  {#each data as { option, n, n_perc }}
    <div class="mt-2 flex">
      <p class="w-28 py-2 text-right font-semibold">
        {option}
      </p>
      <div class="ml-3 flex-1 rounded border-2 border-gray-800 bg-gray-300">
        {#if max_n > 0}
          <div
            class={`h-full rounded transition-all ${
              n_perc > 60
                ? "bg-green-600"
                : n_perc > 30
                ? "bg-yellow-400"
                : "bg-red-600"
            }`}
            style:width={`${n_perc}%`}
          />
        {/if}
      </div>
      <div class="w-6 self-center text-center text-2xl font-bold">
        {n}
      </div>
    </div>
  {/each}
</div>
