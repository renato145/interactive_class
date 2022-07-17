<script lang="ts">
  import type { CupColor } from "bindings/CupColor";

  export let cups: number,
    cupPerc: number,
    color: CupColor = null;
</script>

<div
  class={`flex justify-center items-center w-full transition-all group ${
    color === "Red"
      ? "bg-red-500"
      : color === "Yellow"
      ? "bg-yellow-300"
      : color === "Green"
      ? "bg-green-500"
      : "bg-gray-300"
  }`}
  style:height={`${100 * cupPerc}%`}
>
  {#if cupPerc > 0}
    <p
      class="text-3xl font-bold text-gray-800 opacity-0 group-hover:opacity-100 transition"
    >
      {cups} cups ({(100 * cupPerc).toFixed(2)}%)
    </p>
  {/if}
</div>
