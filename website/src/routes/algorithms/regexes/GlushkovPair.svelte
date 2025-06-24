<script lang="ts">
  import type { GlState } from "@gregofi1/regex-tooling";
  const { pair }: { pair: GlState } = $props();
</script>

<span>{pair[0]}</span><span class="text-sm align-bottom">{pair[1]}</span>
