<script lang="ts">
	import MonacoEditor from '$lib/monaco/MonacoEditor.svelte';
	import Spinner from '$lib/Spinner.svelte';

	let stdin: HTMLElement;
	let stdout: HTMLElement;
	let editor: MonacoEditor;
	let loading = false;

	const compile = async () => {
		const code = editor.getEditorValue();
		loading = true;
		const response = await fetch('/api/code-eval', {
			method: 'POST',
			body: JSON.stringify({
				code,
				language: 'Lua'
			}),
			mode: 'cors',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json'
			}
		});
		if (response.ok) {
			const data = await response.json();
			stdin.innerText = data.stdout;
			stdout.innerText = data.stderr;
		} else {
			stdin.innerText = 'Error communicating with the evaluating server';
			stdout.innerText = 'Error communicating with the evaluating server';
		}
		loading = false;
	};
</script>

<div class="flex flex-row max-xl:flex-col grow">
	<div class="border border-gray-300 grow flex flex-col">
		<div class="ml-2 h-10 flex items-center">
			<div>
				<button class="btn btn-blue" on:click={compile}>Run!</button>
			</div>
		</div>
		<div class="grow">
			<MonacoEditor bind:this={editor} />
		</div>
	</div>
	<div class="xl:w-1/2 max-xl:h-1/3 flex flex-col">
		<div
			class="relative border font-mono p-2 border-gray-300 h-1/2 {loading ? 'bg-slate-200' : ''}"
		>
			<div bind:this={stdin} />
			{#if loading}
				<Spinner />
			{/if}
		</div>
		<div
			class="relative border font-mono p-2 border-gray-300 h-1/2 {loading ? 'bg-slate-200' : ''}"
		>
			<div bind:this={stdout} />
			{#if loading}
				<Spinner />
			{/if}
		</div>
	</div>
</div>

<style>
	:global(body) {
		height: 100%;
	}
	:global(html) {
		height: 100%;
	}
	:global(#main-div) {
		height: 100%;
	}

	.btn {
		@apply font-bold py-2 px-4 rounded;
	}
	.btn-blue {
		@apply bg-green-700 text-white;
	}
	.btn-blue:hover {
		@apply bg-green-900;
	}
</style>
