export type ILanguage = {
	name: string;
	// Name of the language as used by the server
	server_name: string;
	// Name of the language as used by the monaco editor.
	// This can be different from the actual language,
	// for example for Racket we use Scheme.
	editor_name: string;
	// The text representation, what is shown in the dropdown.
	text: string;
	executors?: string[];
	compilers?: string[];
};

export type Result = {
	stdout: string;
	stderr: string;
};

/// What the user has currently selected.
/// BEWARE: This is also saved together with
/// the sent code in the URL, so everything
/// you add warants a change of all items
/// in the DB, unless you make it optional.
export type Selection = {
	language: LangKey;
	executor?: string;
	compiler?: string;
	compilerOptions?: string;
};

/// Data that are saved in the DB under links
export type LinkData = {
	selection: Selection;
	code: string;
};

export type LangKey =
	| 'lua'
	| 'python3'
	| 'racket'
	| 'bash'
	| 'c'
	| 'cpp'
	| 'rust'
	| 'mjolnir'
	| 'haskell';
