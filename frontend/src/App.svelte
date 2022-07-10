<script lang="ts">
  import type { CupsInfo } from "bindings/CupsInfo";

  const getCups = async () => {
    const d = await fetch("/cups").then((response) => response.json());
    return d as CupsInfo;
  };
  let cups = getCups();
</script>

<div class="container mx-auto mt-4 rounded bg-slate-50 p-4">
  <div class="text-6xl">Interactive Class</div>

  <div class="mt-8">
    {#await cups}
      <p>loading...</p>
    {:then data}
      <p class="text-3xl"><span class="font-bold">Rooms:</span> {data.rooms}</p>
    {:catch error}
      <p>An error occurred: {error}</p>
    {/await}
  </div>
</div>
