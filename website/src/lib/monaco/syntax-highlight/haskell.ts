import * as monaco from 'monaco-editor';

// TODO: This is definitely not a correct version
// but it at least provides some colors...
const syntax = (): monaco.languages.IMonarchLanguage => ({
	keywords: [
		'as',
		'case',
		'of',
		'class',
		'data',
		'default',
		'deriving',
		'do',
		'forall',
		'if',
		'then',
		'else',
		'import',
		'let',
		'in',
		'where'
	],

	typeKeywords: ['Int', 'Real', 'String'],

	operators: [
		'!',
		"'",
		'"',
		'-',
		'--',
		'-<',
		'-<<',
		'->',
		'::',
		';',
		'<-',
		',',
		'=',
		'=>',
		'>',
		'?',
		'#',
		'*',
		'@',
		'\\',
		'_',
		'`',
		'|',
		'~',
		'$'
	],

	// we include these common regular expressions
	symbols: /[=><!~?:&|+\-*/^%]+/,

	// C# style strings
	escapes: /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

	// The main tokenizer for our languages
	tokenizer: {
		root: [
			// identifiers and keywords
			[
				/[a-z_$][\w$]*/,
				{ cases: { '@typeKeywords': 'keyword', '@keywords': 'keyword', '@default': 'identifier' } }
			],
			[/[A-Z][\w$]*/, 'type.identifier'], // to show class names nicely

			// whitespace
			{ include: '@whitespace' },

			// delimiters and operators
			[/[{}()[\]]/, '@brackets'],
			[/[<>](?!@symbols)/, '@brackets'],
			[/@symbols/, { cases: { '@operators': 'operator', '@default': '' } }],

			// numbers
			[/[0-9]+/, 'number'],

			// strings
			[/"([^"\\]|\\.)*$/, 'string.invalid'], // non-teminated string
			[/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],

			// characters
			[/'[^\\']'/, 'string'],
			[/(')(@escapes)(')/, ['string', 'string.escape', 'string']],
			[/'/, 'string.invalid']
		],

		string: [
			[/[^\\"]+/, 'string'],
			[/@escapes/, 'string.escape'],
			[/\\./, 'string.escape.invalid'],
			[/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }]
		],

		whitespace: [
			[/[ \t\r\n]+/, 'white'],
			[/--.*$/, 'comment']
		]
	}
});

monaco.languages.register({ id: 'haskell' });
monaco.languages.setMonarchTokensProvider('haskell', syntax());
