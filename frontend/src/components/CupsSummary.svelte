<script lang="ts">
  import CupBlock from "./CupBlock.svelte";
  export let greenCups, yellowCups, redCups, total;
  $: unanswered = total - greenCups - yellowCups - redCups;
</script>

<div class="w-[300px] h-[400px] rounded shadow ring ring-gray-500">
  <CupBlock color="Red" cupPerc={redCups / total} cups={redCups} />
  <CupBlock color="Yellow" cupPerc={yellowCups / total} cups={yellowCups} />
  <CupBlock color="Green" cupPerc={greenCups / total} cups={greenCups} />
  <CupBlock cupPerc={unanswered / total} cups={unanswered} />
</div>
