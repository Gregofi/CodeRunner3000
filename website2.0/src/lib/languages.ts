export type Executor = {
    // Server name of the interpreter
    value: string;
    // Display name of the interpreter
    label: string;
};

export type LanguageConfig = {
    name: string;
    label: string;
    server_name: string;
    interpreters?: Executor[];
    compilers?: Executor[];
};

export const languages: Record<string, LanguageConfig> = {
    lua: {
        name: 'lua',
        label: 'Lua',
        server_name: 'lua',
        interpreters: [
            { value: 'lua5.1.5', label: '5.1' },
            { value: 'lua5.2.4', label: '5.2' },
            { value: 'lua5.3.6', label: '5.3' },
            { value: 'lua5.4.6', label: '5.4' },
        ],
    },
    python: {
        name: 'python',
        label: 'Python',
        server_name: 'python',
        interpreters: [{ value: 'python-bookworm', label: 'Debian Bookworm' }],
    },
    cpp: {
        name: 'cpp',
        label: 'C++',
        server_name: 'cpp',
        compilers: [{ value: 'gcc-bookworm', label: 'GCC - Debian Bookworm' }],
    },
};

export type Language = keyof typeof languages;
