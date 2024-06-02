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

export type LangKey = 'lua' | 'python3' | 'racket' | 'bash' | 'c' | 'cpp' | 'rust';
