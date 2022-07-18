<script lang="ts">
  import type { Question } from "bindings/Question";

  export let createQuestion: (question: Question) => void;
  let creating = false,
    title = "",
    options = ["", ""];

  const submitCreateQuestion = () => {
    createQuestion({
      title,
      options,
    });
    creating = false;
  };
  const addOption = () => {
    options = [...options, `Answer ${options.length + 1}`];
    console.log(options);
  };
</script>

<button class="btn" on:click={() => (creating = true)} disabled={creating}
  >Create new question</button
>

{#if creating}
  <form class="mt-2" on:submit|preventDefault={submitCreateQuestion}>
    <label for="title">Question title:</label>
    <input
      type="text"
      id="title"
      name="title"
      placeholder="Question title"
      bind:value={title}
      required
    />
    <p class="mt-4 text-lg">Answer options:</p>
    {#each options as option, i}
      <div class="mt-2">
        <label for={`option${i + 1}`}>Option {i + 1}:</label>
        <input
          type="text"
          id={`option${i + 1}`}
          name={`option${i + 1}`}
          placeholder={`Answer Option ${i + 1}`}
          bind:value={option}
          required
        />
      </div>
    {/each}
    <button
      class="btn block mt-2"
      type="button"
      on:click|preventDefault={() => addOption()}>Add answer option</button
    >
    <button class="btn mt-4" type="submit">Create question</button>
  </form>
{/if}
