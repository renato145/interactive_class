<script lang="ts">
  import type { WSData } from "../stores/ws";
  export let status: WSData["status"];
  let klass = "";
  export { klass as class };
</script>

<div
  class={`h-4 w-4 rounded-full shadow-xl ${
    status === "connected"
      ? "bg-green-600"
      : status === "working"
      ? "bg-yellow-500"
      : "bg-red-500"
  } ${klass}`}
/>
