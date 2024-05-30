'use client';

import { Header } from './components/header';
import { CodeOutput } from './components/codeOutput';
import { CheckBox } from './components/checkBox';

import { languages, Language } from './lib/languages';

import { initVimMode } from 'monaco-vim';
import Editor from '@monaco-editor/react';
import React from 'react';
import Select from 'react-select';

type CurrentChoice = {
    name: Language;
    interpreter?: string;
    compiler?: string;
};

export default function Home() {
    // TODO: Maybe make this an object (migrate langObject to this)
    let [lang, setLang] = React.useState<CurrentChoice>({ name: 'lua', interpreter: 'lua5.4.6' });

    const langObject = languages[lang.name];
    const currentInterpreter = langObject?.interpreters?.find((i) => i.value === lang.interpreter);
    const currentCompiler = langObject?.compilers?.find((i) => i.value === lang.compiler);
    let vimMode = React.useRef<ReturnType<typeof initVimMode>>();

    const editorRef = React.useRef<typeof Editor>();

    function handleEditorDidMount(editor: typeof Editor, monaco: any) {
        editorRef.current = editor;
    }

    if (!langObject) {
        return <div>Language not found</div>;
    }

    return (
        <div className="flex h-full flex-col">
            <Header />
            <main className="flex flex-col grow">
                <div className="grow flex flex-col">
                    <div className="flex flex-row">
                        <Select
                            value={langObject}
                            getOptionValue={(o) => o.name}
                            options={Object.values(languages)}
                            onChange={(conf) =>
                                setLang({
                                    ...lang,
                                    name: conf!.name,
                                    interpreter: conf!.interpreters?.at(0)?.value,
                                    compiler: conf!.compilers?.at(0)?.value,
                                })
                            }
                        />
                        {currentInterpreter && (
                            <Select
                                value={currentInterpreter}
                                options={langObject.interpreters}
                                onChange={(opt) => setLang({ ...lang, interpreter: opt!.value })}
                            />
                        )}
                        {currentCompiler && (
                            <Select
                                value={currentCompiler}
                                options={langObject.compilers}
                                onChange={(opt) => setLang({ ...lang, compiler: opt!.value })}
                            />
                        )}
                        <CheckBox
                            onChange={(checked: boolean) => {
                                if (checked) {
                                    vimMode.current = initVimMode(editorRef.current!);
                                } else {
                                    vimMode.current?.dispose();
                                    vimMode.current = undefined;
                                }
                            }}
                            label="Vim mode"
                        />
                    </div>
                    <div className="grow">
                        <Editor
                            height="100%"
                            defaultLanguage="javascript"
                            defaultValue="// some comment"
                            onMount={handleEditorDidMount}
                        />
                    </div>
                </div>
                <div className="flex flex-col h-1/3">
                    <CodeOutput code="console.log('Hello, world!');" />
                    <CodeOutput code="console.log('Hello, world!');" />
                </div>
            </main>
        </div>
    );
}
