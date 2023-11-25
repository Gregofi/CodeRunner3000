<script lang="ts">
	import { onMount } from 'svelte';
	import MonacoEditor from '$lib/monaco/MonacoEditor.svelte';
	import Spinner from '$lib/Spinner.svelte';

	interface ILanguage {
		name: string;
		// Name of the language as used by the server
		server_name: string;
		// Name of the language as used by the monaco editor.
		// THis can be different than the actual language,
		// for example for Racket we use Scheme.
		editor_name: string;
		// The text representation, what is shown in the dropdown.
		text: string;
	}

	const languages: ILanguage = {
		lua: { name: 'lua', server_name: 'lua5.1', editor_name: 'lua', text: 'Lua 5.1' },
		python3: { name: 'python3', server_name: 'python3', editor_name: 'python', text: 'Python 3' },
		racket: { name: 'racket', server_name: 'racket', editor_name: 'scheme', text: 'Racket' },
		bash: { name: 'bash', server_name: 'bash', editor_name: 'shell', text: 'Bash' },
		c: { name: 'c', server_name: 'c', editor_name: 'c', text: 'C' },
		cpp23gcc: { name: 'cpp23gcc', server_name: 'cpp23gcc', editor_name: 'cpp', text: 'C++23 GCC' }
	};

	let stdout: HTMLElement;
	let stderr: HTMLElement;
	let editor: MonacoEditor;
	let loading = false;
	let timer;
	let current_language = 'lua';
	const delay = 1000;

	const compile = async () => {
		const code = editor.getEditorValue();
		loading = true;
		const language = languages[current_language].server_name;
		const response = await fetch('/api/code-eval', {
			method: 'POST',
			body: JSON.stringify({
				code,
				language
			}),
			mode: 'cors',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json'
			}
		});
		if (response.ok) {
			const data = await response.json();
			stdout.innerText = data.stdout;
			stderr.innerText = data.stderr;
		} else {
			stdout.innerText = 'Error communicating with the evaluating server';
			stderr.innerText = 'Error communicating with the evaluating server';
		}
		loading = false;
	};

	const setEditorDebounce = () => {
		if (!editor) {
			return;
		}
		editor.onDidChangeContent(() => {
			clearTimeout(timer);
			timer = setTimeout(() => {
				compile();
			}, delay);
		});
	};

	const languageChange = () => {
		const language = languages[current_language].editor_name;
		editor.changeLanguage(language);
	};

	onMount(() => {
		window.addEventListener('editor-loaded', () => {
			setEditorDebounce();
			window.addEventListener('keydown', (e) => {
				if (e.ctrlKey && e.key === 's') {
					clearTimeout(timer);
					e.preventDefault();
					compile();
				}
			});
			compile();
		});
	});
</script>

<div class="flex flex-row max-xl:flex-col grow">
	<div class="border border-gray-300 grow flex flex-col">
		<div class="ml-2 h-10 flex items-center">
			<div>
				<button class="btn btn-blue" on:click={compile}>Run (Ctrl+S)</button>
				<select
					bind:value={current_language}
					on:change={languageChange}
					name="language"
					class="ml-2"
				>
					{#each Object.values(languages) as language}
						<option value={language.name}>{language.text}</option>
					{/each}
				</select>
			</div>
		</div>
		<div class="grow">
			<MonacoEditor bind:this={editor} />
		</div>
	</div>
	<div class="xl:w-1/2 max-xl:h-1/3 flex flex-col">
		<div
			class="relative overflow-auto border font-mono p-2 border-gray-300 h-1/2 {loading
				? 'bg-slate-200'
				: ''}"
		>
			<pre bind:this={stdout} />
			{#if loading}
				<Spinner />
			{/if}
		</div>
		<div
			class="relative overflow-auto border font-mono p-2 border-gray-300 h-1/2 {loading
				? 'bg-slate-200'
				: ''}"
		>
			<pre bind:this={stderr} />
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
