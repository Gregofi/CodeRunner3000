<script lang="ts">
	import { onMount } from 'svelte';
	import MonacoEditor from '$lib/monaco/MonacoEditor.svelte';
	import Spinner from '$lib/Spinner.svelte';
	import Modal from '$lib/Modal.svelte';
	import { defaultPrograms } from '$lib/constants';
	import { getSettings, setVimMode } from '$lib/settings';

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
		executors?: string[];
		compilers?: string[];
	}

	interface IPayload {
		code: string;
		language: string;
		executor?: string;
		compiler?: string;
	}

	const languages: { [key in string]: ILanguage } = {
		lua: {
			name: 'lua',
			server_name: 'lua',
			editor_name: 'lua',
			text: 'Lua',
			executors: ['lua5.4.6', 'lua5.3.6', 'lua5.2.4', 'lua5.1.5']
		},
		python3: {
			name: 'python3',
			server_name: 'python3',
			editor_name: 'python',
			text: 'Python 3',
			executors: ['python3-bookworm']
		},
		// racket: { name: 'racket', server_name: 'racket', editor_name: 'scheme', text: 'Racket', executors: ["racket-bookworm"] },
		// bash: { name: 'bash', server_name: 'bash', editor_name: 'shell', text: 'Bash', executors: ["bash"] },
		c: {
			name: 'c',
			server_name: 'c',
			editor_name: 'c',
			text: 'C',
			compilers: ['gcc-trunk', 'gcc-bookworm']
		},
		cpp: {
			name: 'cpp',
			server_name: 'cpp',
			editor_name: 'cpp',
			text: 'C++',
			compilers: ['gcc-bookworm']
		},
		// haskell: { name: 'haskell', server_name: 'haskell', editor_name: 'Haskell', text: 'Haskell', compilers: ["ghc-bookworm"] },
		rust: {
			name: 'rust',
			server_name: 'rust',
			editor_name: 'rust',
			text: 'Rust',
			compilers: ['rustc-bookworm']
		}
	};

	let stdout: HTMLElement;
	let stderr: HTMLElement;
	let editor: MonacoEditor;
	let loading = false;
	let timer: ReturnType<typeof setTimeout>;
	let current_language = 'lua';
	let current_executor: string | undefined;
	let current_compiler: string | undefined;
	let showModal = false;
	let lastUrl = '';
	let vimChecker: HTMLInputElement;
	const delay = 1000;

	const createPayload = (): IPayload => {
		const code = editor.getEditorValue();
		const language = languages[current_language];
		const payload: IPayload = {
			code,
			language: language.server_name,
			// the current_x stays even when changing to language that
			// has no compiler/interpreter, so chech if the language even
			// needs interpreter/compiler.
			compiler: language.compilers ? current_compiler : undefined,
			executor: language.executors ? current_executor : undefined
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
		programs[current_language] = editor.getEditorValue();
		localStorage.setItem('saved_programs', JSON.stringify(programs));
	};

	const loadFromLocalStorage = () => {
		const savedCode = localStorage.getItem('saved_programs');
		if (savedCode) {
			const programs = JSON.parse(savedCode);
			const code = programs[current_language];
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
		const editor_name = languages[current_language].editor_name;
		const defaultCode = defaultPrograms[editor_name];
		if (defaultCode) {
			editor.setEditorValue(defaultCode);
		} else {
			console.log('Unable to found default program for language: ' + current_language);
		}
	};

	const languageChange = () => {
		current_compiler = languages[current_language].compilers?.[0];
		current_executor = languages[current_language].executors?.[0];
		const language = languages[current_language].editor_name;
		editor.changeLanguage(language);
		const loaded = loadFromLocalStorage();
		if (!loaded) {
			renderDefaultCode();
		}
	};

	const createLink = () => {
		const urlParams = new URLSearchParams();
		const payload = createPayload();
		urlParams.set('input', btoa(JSON.stringify(payload)));
		const url = `${window.location.origin}${window.location.pathname}?${urlParams.toString()}`;
		if (url.length > 2000) {
			console.log('URL too long, might not be supported by some browsers.');
		}
		return url;
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
				const input = JSON.parse(atob(codedInput));
				const code = input.code;
				const language = input.language;
				if (code && language && languages[language] !== undefined) {
					current_executor = input.exec_opts?.executor;
					current_compiler = input.compiler_opts?.compiler;
					current_language = language;
					languageChange();
					editor.setEditorValue(code);
				}
			} else {
				const loadedFromLocal = loadFromLocalStorage();
				if (!loadedFromLocal) {
					renderDefaultCode();
				}
				// And overwrite it with the saved program if it exists.
			}
		});
	});

	const changeButtonText = (elem: HTMLButtonElement, text: string) => {
		let oldText = elem.innerText;
		elem.innerText = text;
		elem.disabled = true;
		elem.style.webkitFilter = 'grayscale(1)';
		elem.style.cursor = 'not-allowed';
		setTimeout(() => {
			elem.innerText = oldText;
			elem.disabled = false;
			elem.style.webkitFilter = 'grayscale(0)';
			elem.style.cursor = 'pointer';
		}, 2000);
	};
</script>

<div class="flex flex-row max-xl:flex-col grow">
	<div class="border border-gray-300 grow flex flex-col">
		<div class="ml-2 h-10 flex items-center overflow-x-auto">
			<button class="btn btn-blue whitespace-nowrap" on:click={compile}>Run (Ctrl+S)</button>
			<select bind:value={current_language} on:change={languageChange} name="language" class="ml-2">
				{#each Object.values(languages) as language}
					<option value={language.name}>{language.text}</option>
				{/each}
			</select>
			{#if languages[current_language].executors?.length > 0}
				<select bind:value={current_executor} name="executor" class="ml-2">
					{#each languages[current_language].executors as executor}
						<option value={executor}>{executor}</option>
					{/each}
				</select>
			{/if}
			{#if languages[current_language].compilers?.length > 0}
				<select bind:value={current_compiler} name="compiler" class="ml-2">
					{#each languages[current_language].compilers as compiler}
						<option value={compiler}>{compiler}</option>
					{/each}
				</select>
			{/if}
			<button
				class="btn"
				on:click={() => {
					showModal = true;
					lastUrl = createLink();
				}}>Share</button
			>
			<input type="checkbox" name="vim-mode" on:change={toggleVimMode} bind:this={vimChecker} />
			<span class="font-bold ml-1">Vim</span>
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

<Modal bind:showModal>
	<div slot="header">
		<h1 class="text-xl font-bold">Share</h1>
	</div>
	<p>Please keep in mind that this feature is still in alpha and subject to change.</p>
	<p>This means that the code might not work in the future.</p>
	<br />
	<p>Use the following link to share your code:</p>
	<input class="border w-96 p-1 rounded-lg" type="text" value={lastUrl} readonly />
	<button
		class="btn btn-blue mt-2 w-44"
		on:click={(e) => {
			navigator.clipboard.writeText(lastUrl);
			changeButtonText(e.target, 'Copied!');
		}}>Copy to clipboard</button
	>
	{#if lastUrl.length > 2048}
		<p class="text-red-500">Warning: URL is too long, might not be supported by some browsers.</p>
	{/if}
</Modal>

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
		@apply font-bold py-2 px-4;
	}
	.btn-blue {
		@apply bg-green-700 text-white;
	}
	.btn-blue:hover {
		@apply bg-green-900;
	}
</style>
