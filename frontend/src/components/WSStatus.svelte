<script lang="ts">
  import { wsStatusStore } from "../stores/ws";
  let klass = "";
  export { klass as class };
</script>

<div
  class={`h-4 w-4 rounded-full shadow-xl ${
    $wsStatusStore === "connected2room"
      ? "bg-green-600"
      : $wsStatusStore === "working"
      ? "bg-yellow-500"
      : "bg-red-500"
  } ${klass}`}
/>
