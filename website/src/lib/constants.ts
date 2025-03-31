import type { ILanguage, LangKey } from './types';

export const languages: { [key in LangKey]: ILanguage } = {
	lua: {
		name: 'lua',
		server_name: 'lua',
		editor_name: 'lua',
		text: 'Lua',
		executors: ['lua5.4.6', 'lua5.3.6', 'lua5.2.4', 'lua5.1.5']
	},
	js: {
		name: 'js',
		server_name: 'js',
		editor_name: 'javascript',
		text: 'Javascript',
		executors: ['nodejs-bookworm']
	},
	python3: {
		name: 'python3',
		server_name: 'python3',
		editor_name: 'python',
		text: 'Python 3',
		executors: ['python3-bookworm']
	},
	racket: {
		name: 'racket',
		server_name: 'racket',
		editor_name: 'scheme',
		text: 'Racket',
		executors: ['racket-v8.11.1']
	},
	bash: {
		name: 'bash',
		server_name: 'bash',
		editor_name: 'shell',
		text: 'Bash',
		executors: ['bash-bookworm']
	},
	c: {
		name: 'c',
		server_name: 'c',
		editor_name: 'c',
		text: 'C',
		compilers: ['gcc-bookworm']
	},
	cpp: {
		name: 'cpp',
		server_name: 'cpp',
		editor_name: 'cpp',
		text: 'C++',
		compilers: ['gcc-bookworm']
	},
	haskell: {
		name: 'haskell',
		server_name: 'haskell',
		editor_name: 'haskell',
		text: 'Haskell',
		compilers: ['ghc-bookworm']
	},
	rust: {
		name: 'rust',
		server_name: 'rust',
		editor_name: 'rust',
		text: 'Rust',
		compilers: ['rustc-bookworm']
	},
	mjolnir: {
		name: 'mjolnir',
		server_name: 'mjolnir',
		editor_name: 'mjolnir',
		text: 'Mjolnir',
		executors: ['master']
	}
};
