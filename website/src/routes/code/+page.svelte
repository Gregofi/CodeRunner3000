<script lang="ts">
	import { onMount } from 'svelte';
	import MonacoEditor from '$lib/monaco/MonacoEditor.svelte';
	import OutputBox from '$lib/OutputBox.svelte';
	import { defaultPrograms } from '$lib/defaultPrograms';
	import { getSettings, setVimMode } from '$lib/settings';
	import type { Result, LangKey } from '$lib/types';
	import { languages } from '$lib/constants';
	import { sendCodeToServer } from '$lib/remoteUtils';

	let editor: MonacoEditor;

	let loading = false;
	let lastResult: Result = {
		stdout: '',
		stderr: ''
	};
	/// Only last compilation can overwrite the result, otherwise slower code
	/// executed earlier could overwrite result from faster code executed later.
	let lastResultId = 0;

	let timer: ReturnType<typeof setTimeout>;

	// It would be nice if we could bind these guys together into an object,
	// but it seems that the bind:value things doesn't really work with it.
	let currentLanguage: LangKey = 'lua';
	let currentExecutor: string | undefined;
	let currentCompiler: string | undefined;
	let compilerOptions: string | undefined;

	$: langObj = languages[currentLanguage];

	let vimChecker: HTMLInputElement;

	const delay = 2000;

	const compile = () => {
		if (timer) {
			clearTimeout(timer);
		}
		loading = true;
		const id = ++lastResultId;
		sendCodeToServer(
			editor.getEditorValue(),
			currentLanguage,
			currentCompiler,
			currentExecutor,
			compilerOptions
		)
			.then((result: Result) => {
				if (lastResultId === id) {
					lastResult = result;
				}
			})
			.catch(() => {
				lastResult = {
					stdout: 'Server error; Could not run the code.',
					stderr: ''
				};
			})
			.finally(() => {
				loading = false;
			});
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
		if (!editor) {
			return null;
		}
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

			// Even though we do this very often, keep it.
			// This way it works if user accidentally mistypes and
			// closes the tab.
			saveToLocalStorage();
		});
	};

	const renderDefaultCode = () => {
		if (!editor) {
			return;
		}
		const editor_name = langObj.editor_name;
		const defaultCode = defaultPrograms[editor_name];
		if (defaultCode) {
			editor.setEditorValue(defaultCode);
		} else {
			console.log('Unable to found default program for language: ' + currentLanguage);
		}
	};

	const languageChange = (conf: { compiler?: string; executor?: string } = {}) => {
		langObj = languages[currentLanguage];
		currentCompiler = conf.compiler ?? langObj.compilers?.[0];
		currentExecutor = conf.executor ?? langObj.executors?.[0];
		compilerOptions = '';
		const language = languages[currentLanguage].editor_name;
		editor.changeLanguage(language);
		const loaded = loadFromLocalStorage();
		if (!loaded) {
			renderDefaultCode();
		}
		compile();
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
			// Compile on Ctrl+s
			window.addEventListener('keydown', (e) => {
				if (e.ctrlKey && e.key === 's') {
					clearTimeout(timer);
					e.preventDefault();
					compile();
				}
			});

			if (settings.vimMode) {
				vimChecker.checked = true;
				editor.turnOnVimMode();
			}

			// And overwrite it with the saved program if it exists.
			const loadedFromLocal = loadFromLocalStorage();
			if (!loadedFromLocal) {
				renderDefaultCode();
			}
			compile();

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
			<select
				bind:value={currentLanguage}
				on:change={() => languageChange()}
				name="language"
				class="ml-2"
			>
				{#each Object.values(languages) as language}
					<option value={language.name}>{language.text}</option>
				{/each}
			</select>
			{#if langObj.executors?.length ?? 0 > 0}
				<select bind:value={currentExecutor} name="executor" class="ml-2">
					{#each langObj.executors ?? [] as executor}
						<option value={executor}>{executor}</option>
					{/each}
				</select>
			{/if}
			{#if langObj.compilers?.length ?? 0 > 0}
				<select bind:value={currentCompiler} name="compiler" class="ml-2">
					{#each langObj.compilers ?? [] as compiler}
						<option value={compiler}>{compiler}</option>
					{/each}
				</select>
			{/if}
			<input
				class="ml-1"
				type="checkbox"
				name="vim-mode"
				on:change={toggleVimMode}
				bind:this={vimChecker}
			/>
			<span class="font-bold ml-1">Vim</span>
			{#if langObj.compilers}
				<input
					class="ml-2 p-1 border"
					bind:value={compilerOptions}
					placeholder="compiler options..."
				/>
			{/if}
		</div>
		<div class="grow data-pw-monaco-editor-main">
			<MonacoEditor bind:this={editor} />
		</div>
	</div>
	<OutputBox isRequestPending={loading} {lastResult} />
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
