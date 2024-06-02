<script lang="ts">
	import Spinner from '$lib/Spinner.svelte';
	import type { Result } from '$lib/types';

	export let isRequestPending = false;
	export let lastResult: Result | null = null;
</script>

<div class="xl:w-1/2 max-xl:h-1/3 flex flex-col">
	<div
		class="relative overflow-auto border font-mono p-2 border-gray-300 h-1/2 {isRequestPending
			? 'bg-slate-200'
			: ''}"
	>
		<pre>{lastResult?.stdout}</pre>
		{#if isRequestPending}
			<Spinner />
		{/if}
	</div>
	<div
		class="relative overflow-auto border font-mono p-2 border-gray-300 h-1/2 {isRequestPending
			? 'bg-slate-200'
			: ''}"
	>
		<pre>{lastResult?.stderr}</pre>
		{#if isRequestPending}
			<Spinner />
		{/if}
	</div>
</div>
