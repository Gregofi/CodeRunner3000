<script lang="ts">
	// Credits from this setup to https://www.codelantis.com/blog/sveltekit-monaco-editor.
	import { onMount, onDestroy } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
    import { initVimMode } from 'monaco-vim';

	let editor: Monaco.editor.IStandaloneCodeEditor;
    let vim_mode: ReturnType<initVimMode> | null = null;
	let monaco: typeof Monaco;
	function getEditorValue() {
		return editor.getValue();
	}

	function setEditorValue(code: string) {
		editor.setValue(code);
	}

	function getUnderlyingEditor() {
		return editor;
	}

	function onDidChangeContent(callback: () => void) {
		editor.onDidChangeModelContent(callback);
	}

	function changeLanguage(language: string) {
        const model = editor.getModel();
        if (model !== null) {
            monaco.editor.setModelLanguage(model, language);
        }
	}

    function turnOnVimMode() {
        if (vim_mode === null) {
            vim_mode = initVimMode(editor);
        }
    }

    function turnOffVimMode() {
        if (vim_mode !== null) {
            vim_mode.dispose();
            vim_mode = null;
        }
    }

	let editorContainer: HTMLElement;
	let editorParent: HTMLDivElement;

	onMount(async () => {
		monaco = (await import('./monaco')).default;

		editor = monaco.editor.create(editorContainer);
		const model = monaco.editor.createModel('', 'lua');
		editor.setModel(model);

		// A hacky? workaround to resize the editor with the page,
		// since Monaco doesn't do this automatically.
		// We could use 'automaticLayout' config option but that
		// only works when making the page bigger.
		window.addEventListener('resize', () => {
			editor?.layout({ width: 0, height: 0 });
			// Resize when the frame refreshes
			window.requestAnimationFrame(() => {
				editor.layout(editorParent.getBoundingClientRect());
			});
		});

		const event = new CustomEvent('editor-loaded', {
			detail: {
				editor: editor
			}
		});

		window.dispatchEvent(event);
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});

	export {
		getEditorValue,
		setEditorValue,
		getUnderlyingEditor,
		onDidChangeContent,
		changeLanguage,
        turnOnVimMode,
        turnOffVimMode,
	};
</script>

<div class="h-full" bind:this={editorParent}>
	<div class="h-full" bind:this={editorContainer} />
</div>
