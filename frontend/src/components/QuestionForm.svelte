<script lang="ts">
  import type { Question } from "bindings/Question";
  import { t } from "svelte-i18n";

  export let createQuestion: (question: Question) => void;
  let creating = false,
    title = "",
    options = ["", ""];

  const cancelQuestionCreate = () => {
    creating = false;
    title = "";
    options = ["", ""];
  };

  const submitCreateQuestion = () => {
    createQuestion({
      title,
      options,
    });
    creating = false;
    title = "";
    options = ["", ""];
  };

  const addOption = () => {
    options = [...options, ""];
  };
</script>

<button class="btn" on:click={() => (creating = true)} disabled={creating}
  >{$t("create-new-question")}</button
>

{#if creating}
  <form class="mt-2" on:submit|preventDefault={submitCreateQuestion}>
    <label for="title">{$t("question-title")}:</label>
    <input
      class="ml-2"
      type="text"
      id="title"
      name="title"
      placeholder={$t("question-title")}
      bind:value={title}
      required
    />
    <p class="mt-4 text-lg">{$t("answer-options")}:</p>
    {#each options as option, i}
      <div class="mt-2">
        <label for={`option${i + 1}`}>{$t("option")} {i + 1}:</label>
        <input
          class="ml-2"
          type="text"
          id={`option${i + 1}`}
          name={`option${i + 1}`}
          placeholder={`${$t("answer-option")} ${i + 1}`}
          bind:value={option}
          required
        />
      </div>
    {/each}
    <button
      class="btn mt-2 block text-sm"
      type="button"
      on:click|preventDefault={() => addOption()}
      >{$t("add-answer-option")}</button
    >
    <button class="btn mt-4 text-lg" type="submit"
      >{$t("create-question")}</button
    >
    <button
      class="btn-cancel"
      type="button"
      on:click|preventDefault={cancelQuestionCreate}>{$t("cancel")}</button
    >
  </form>
{/if}
