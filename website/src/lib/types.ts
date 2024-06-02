export type ILanguage = {
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
};

export type IPayload = {
	code: string;
	language: string;
	executor?: string;
	compiler?: string;
	executor_args?: string[];
	compiler_args?: string[];
	program_args?: string[];
	stdin?: string;
};
