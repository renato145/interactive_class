<script lang="ts">
  import WsError from "../components/WSError.svelte";
  import WsStatus from "../components/WSStatus.svelte";
  import CupsSummary from "../components/CupsSummary.svelte";
  import QuestionForm from "../components/QuestionForm.svelte";
  import QuestionViewAll from "../components/QuestionViewAll.svelte";
  import { getWSStore } from "../stores/ws";
  export let roomName;

  let { wsStore, createQuestion, deleteQuestion, publishQuestion } = getWSStore(
    roomName,
    "Teacher"
  );
</script>

<div>
  <div class="flex align-bottom">
    <WsStatus status={$wsStore.status} class="self-center" />
    <p class="ml-3 text-3xl">
      Room: {roomName} ({$wsStore.connections === 0
        ? "no students"
        : $wsStore.connections === 1
        ? "1 student"
        : `${$wsStore.connections} students`}) -
      <a href={`/room/${roomName}`} target="_black" rel="noopener"
        >[student view]</a
      >
    </p>
  </div>
  <div class="mt-4">
    <WsError error_msg={$wsStore.error_msg} />
  </div>

  <!-- Cups visualization -->
  {#if $wsStore.connections > 0}
    <div class="ml-8 mt-8">
      <CupsSummary
        greenCups={$wsStore.cups.green}
        yellowCups={$wsStore.cups.yellow}
        redCups={$wsStore.cups.red}
        total={$wsStore.connections}
      />
    </div>
  {/if}

  <!-- Questions -->
  <div class="mt-8"><QuestionForm {createQuestion} /></div>
  <div class="mt-4">
    <QuestionViewAll
      questions={$wsStore.questions}
      {publishQuestion}
      {deleteQuestion}
      connections={$wsStore.connections}
    />
  </div>
</div>
