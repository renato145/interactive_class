<script lang="ts">
  import type { CupColor } from "bindings/CupColor";

  export let cupPerc: number, color: CupColor;
</script>

<div
  class={`w-full transition-all ${
    color === "Red"
      ? "bg-red-500"
      : color === "Yellow"
      ? "bg-yellow-300"
      : "bg-green-500"
  }`}
  style:height={`${cupPerc * 100}%`}
/>
