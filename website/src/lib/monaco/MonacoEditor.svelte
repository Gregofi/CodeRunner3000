<script lang="ts">
	// Credits from this setup to https://www.codelantis.com/blog/sveltekit-monaco-editor.
	import { onMount, onDestroy } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

	const default_programs = {
		lua: [
			'local function fact(n)',
			'    if n == 0 then',
			'        return 1',
			'    else',
			'        return n * fact(n - 1)',
			'    end',
			'end',
			'',
			'print(fact(5))'
		].join('\n'),
		scheme: [
			'#lang racket',
			'',
			'(define (fact n)',
			'    (if (= n 0)',
			'        1',
			'        (* n (fact (- n 1)))))',
			'',
			'(display (fact 5))'
		].join('\n'),
		python: [
			'def fact(n):',
			'    if n == 0:',
			'        return 1',
			'    else:',
			'        return n * fact(n - 1)',
			'',
			'print(fact(5))'
		].join('\n'),
		shell: ['#!/bin/bash', '', "echo 'Hello, World!'"].join('\n'),
		c: [
			'#include <stdio.h>',
			'',
			'int fact(int n) {',
			'    if (n == 0) {',
			'        return 1;',
			'    } else {',
			'        return n * fact(n - 1);',
			'    }',
			'}',
			'',
			'int main() {',
			'    printf("%d\\n", fact(5));',
			'    return 0;',
			'}'
		].join('\n'),
		cpp: [
			'#include <iostream>',
			'',
			'int fact(int n) {',
			'    if (n == 0) {',
			'        return 1;',
			'    } else {',
			'        return n * fact(n - 1);',
			'    }',
			'}',
			'',
			'int main() {',
			'    std::cout << fact(5) << "\\n";',
			'    return 0;',
			'}'
		].join('\n')
	};

	let editor: Monaco.editor.IStandaloneCodeEditor;
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
		monaco.editor.setModelLanguage(editor.getModel(), language);

		const default_program = default_programs[language];
		if (default_program) {
			setEditorValue(default_programs[language]);
		} else {
			console.log('No default program for language ' + language);
		}
	}

	let editorContainer: HTMLElement;
	let editorParent: HTMLDivElement;

	onMount(async () => {
		monaco = (await import('./monaco')).default;

		editor = monaco.editor.create(editorContainer);
		const model = monaco.editor.createModel(default_programs['lua'], 'lua');
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
		changeLanguage
	};
</script>

<div class="h-full" bind:this={editorParent}>
	<div class="h-full" bind:this={editorContainer} />
</div>
