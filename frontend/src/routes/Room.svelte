<script lang="ts">
  import { onDestroy } from "svelte";
  import type { CupColor } from "bindings/CupColor";
  import type { QuestionPublication } from "bindings/QuestionPublication";
  import QuestionViewStudent from "../components/QuestionViewStudent.svelte";
  import WsError from "../components/WSError.svelte";
  import WsStatus from "../components/WSStatus.svelte";
  import { getWSStore, questionsStore } from "../stores/ws";
  export let roomName;

  let {
    wsStore,
    chooseCup: chooseCup_,
    answerQuestion,
  } = getWSStore(roomName, "Student");
  let color: CupColor;
  const chooseCup = (cupColor: CupColor) => {
    color = cupColor;
    chooseCup_(cupColor);
  };

  let questions: (QuestionPublication & { timeoutID: number })[] = [];
  let unsubscribe = questionsStore.subscribe((question) => {
    if (question !== null) {
      questions
        .filter((d) => d.id === question.id)
        .forEach((d) => {
          clearTimeout(d.timeoutID);
        });

      const timeoutID = setTimeout(() => {
        questions = questions.filter((q) => q.id !== question.id);
      }, question.secs * 1000);
      questions = [
        ...questions.filter((d) => d.id !== question.id),
        { ...question, timeoutID },
      ];
    }
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus status={$wsStore.status} class="self-center" />
    <p class="ml-3 text-3xl">Room: {roomName}</p>
  </div>
  <div class="mt-4">
    <WsError error_msg={$wsStore.error_msg} />
  </div>
  <div class="mt-8">
    <svg
      class={`mx-auto aspect-square w-1/2 max-w-md ${
        color === "Green"
          ? "fill-green-500"
          : color === "Yellow"
          ? "fill-yellow-300"
          : color === "Red"
          ? "fill-red-500"
          : "fill-gray-300"
      }`}
      viewBox="0 0 218 227"
    >
      <path d="M175 1H42L1 226H217L175 1Z" />
    </svg>
  </div>

  <div class="mt-4">
    <div class="mx-auto flex w-1/2 max-w-md flex-col space-y-2">
      <button
        class="rounded bg-green-200 p-2 text-left shadow hover:bg-green-400"
        on:click={() => {
          chooseCup("Green");
        }}
      >
        Green - I am comfortable with my understanding and pacing of the lesson
      </button>

      <button
        class="rounded bg-yellow-200 p-2 text-left shadow hover:bg-yellow-300"
        on:click={() => {
          chooseCup("Yellow");
        }}
      >
        Yellow - I am working through my understanding, I would benefit from the
        teacher slowing down or revisiting the current concept
      </button>

      <button
        class="rounded bg-red-200 p-2 text-left shadow hover:bg-red-400"
        on:click={() => {
          chooseCup("Red");
        }}
      >
        Red - STOP! I am not understanding and I have a question
      </button>
    </div>
  </div>

  {#if questions.length > 0}
    <div class="mt-8 flex flex-wrap gap-8">
      {#each questions as question}
        <QuestionViewStudent
          {question}
          answerQuestion={(i) => answerQuestion(question.id, i)}
        />
      {/each}
    </div>
  {/if}
</div>
