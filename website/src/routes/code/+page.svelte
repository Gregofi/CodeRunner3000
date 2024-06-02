<script lang="ts">
	import { onMount } from 'svelte';
	import MonacoEditor from '$lib/monaco/MonacoEditor.svelte';
	import Spinner from '$lib/Spinner.svelte';
	import { defaultPrograms } from '$lib/defaultPrograms';
	import { getSettings, setVimMode } from '$lib/settings';
	import type { IPayload } from '$lib/types';
	import { languages } from '$lib/constants';

	let stdout: HTMLElement;
	let stderr: HTMLElement;
	let editor: MonacoEditor;
	let loading = false;
	let timer: ReturnType<typeof setTimeout>;

	// It would be nice if we could bind these guys together into an object,
	// but it seems that the bind:value things doesn't really work with it.
	let currentLanguage = 'lua';
	let currentExecutor: string | undefined;
	let currentCompiler: string | undefined;

	$: langObj = languages[currentLanguage];

	let vimChecker: HTMLInputElement;

	const delay = 1000;

	const createPayload = (): IPayload => {
		const code = editor.getEditorValue();
		const payload: IPayload = {
			code,
			language: langObj.server_name,
			// the currentX stays even when changing to language that
			// has no compiler/interpreter, so check if the language even
			// needs interpreter/compiler.
			compiler: langObj.compilers ? currentCompiler : undefined,
			executor: langObj.executors ? currentExecutor : undefined
		};
		return payload;
	};

	const compile = async () => {
		loading = true;
		const body = JSON.stringify(createPayload());
		const response = await fetch('/api/code-eval', {
			method: 'POST',
			body,
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

	const saveToLocalStorage = () => {
		let programsString = localStorage.getItem('saved_programs') ?? '{}';
		let programs = JSON.parse(programsString);
		programs[currentLanguage] = editor.getEditorValue();
		localStorage.setItem('saved_programs', JSON.stringify(programs));
	};

	/// Tries to load saved program from local storage and inserts it
	/// into the editor, if so it returns true. Returns false otherwise.
	const loadFromLocalStorage = () => {
		const savedCode = localStorage.getItem('saved_programs');
		if (savedCode) {
			const programs = JSON.parse(savedCode);
			const code = programs[currentLanguage];
			if (code) {
				editor.setEditorValue(code);
				return true;
			}
		}
		return false;
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

			saveToLocalStorage();
		});
	};

	const renderDefaultCode = () => {
		const editor_name = langObj.editor_name;
		const defaultCode = defaultPrograms[editor_name];
		if (defaultCode) {
			editor.setEditorValue(defaultCode);
		} else {
			console.log('Unable to found default program for language: ' + currentLanguage);
		}
	};

	const languageChange = (conf: { compiler?: string; executor?: string } = {}) => {
		currentCompiler = conf.compiler ?? langObj.compilers?.[0];
		currentExecutor = conf.executor ?? langObj.executors?.[0];
		const language = languages[currentLanguage].editor_name;
		editor.changeLanguage(language);
		const loaded = loadFromLocalStorage();
		if (!loaded) {
			renderDefaultCode();
		}
	};

	const toggleVimMode = (e: Event) => {
		const target = e.target as HTMLInputElement;
		setVimMode(target.checked);
		if (target.checked) {
			editor.turnOnVimMode();
		} else {
			editor.turnOffVimMode();
		}
	};

	onMount(() => {
		window.addEventListener('editor-loaded', () => {
			const settings = getSettings();
			setEditorDebounce();
			window.addEventListener('keydown', (e) => {
				if (e.ctrlKey && e.key === 's') {
					clearTimeout(timer);
					e.preventDefault();
					compile();
				}
			});
			compile();

			if (settings.vimMode) {
				vimChecker.checked = true;
				editor.turnOnVimMode();
			}

			// Check if we have a code in the URL.
			// If not then check if we have a saved program in local storage.
			const urlParams = new URLSearchParams(window.location.search);
			const codedInput = urlParams.get('input');
			if (codedInput !== null) {
				const input = JSON.parse(atob(codedInput)) as IPayload;
				console.log('loading code from url', input);
				const code = input.code;
				const language = input.language;
				if (code && language && langObj !== undefined) {
					currentExecutor = input.executor;
					currentCompiler = input.compiler;
					currentLanguage = language;
					languageChange({ compiler: currentCompiler, executor: currentExecutor });
					editor.setEditorValue(code);
				}
			} else {
				// And overwrite it with the saved program if it exists.
				const loadedFromLocal = loadFromLocalStorage();
				if (!loadedFromLocal) {
					renderDefaultCode();
				}
			}
			// Playwright doesn't have any decent access to monaco, we have to do things
			// like "click the div, ctrl + a, start typing etc.", so this export is
			// to make the test easier to write.
			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-ignore
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			(window as any).getMonacoEditorValue = () => {
				return editor.getEditorValue();
			};
			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-ignore
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			(window as any).setMonacoEditorValue = (value: string) => {
				editor.setEditorValue(value);
			};
		});
	});
</script>

<div id="main-div" class="flex flex-row max-xl:flex-col">
	<div class="border border-gray-300 grow flex flex-col">
		<div class="ml-2 h-10 flex items-center overflow-x-auto">
			<button class="btn btn-blue whitespace-nowrap" on:click={compile}>Run (Ctrl+S)</button>
			<select bind:value={currentLanguage} on:change={languageChange} name="language" class="ml-2">
				{#each Object.values(languages) as language}
					<option value={language.name}>{language.text}</option>
				{/each}
			</select>
			{#if langObj.executors?.length > 0}
				<select bind:value={currentExecutor} name="executor" class="ml-2">
					{#each langObj.executors ?? [] as executor}
						<option value={executor}>{executor}</option>
					{/each}
				</select>
			{/if}
			{#if langObj.compilers?.length > 0}
				<select bind:value={currentCompiler} name="compiler" class="ml-2">
					{#each langObj.compilers ?? [] as compiler}
						<option value={compiler}>{compiler}</option>
					{/each}
				</select>
			{/if}
			<input type="checkbox" name="vim-mode" on:change={toggleVimMode} bind:this={vimChecker} />
			<span class="font-bold ml-1">Vim</span>
		</div>
		<div class="grow data-pw-monaco-editor-main">
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
	.btn {
		@apply font-bold py-2 px-4;
	}
	.btn-blue {
		@apply bg-green-700 text-white;
	}
	.btn-blue:hover {
		@apply bg-green-900;
	}

	#main-div {
		/* The exact size of the rest of the div (3rem is the size of header) */
		height: calc(100vh - 3rem);
	}
</style>
