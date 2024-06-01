'use client';

import { CodeOutput } from './codeOutput';
import { CheckBox } from './checkBox';
import { languages } from '../lib/languages';

import { initVimMode } from 'monaco-vim';
import Editor from '@monaco-editor/react';
import React from 'react';
import Select from 'react-select';
import { CurrentChoice, ExecutionData, ExecutorResponse } from '../lib/types';
import ShareButton from './share';
import { Button } from '@mui/material';

const DEBOUNCE_TIME = 3000;

/// Creates a link through which the current code can be shared.
const createSharedLink = (currentChoices: CurrentChoice, code: string): boolean => {

    navigator.clipboard.writeText("http://localhost:3000/abcd");
    return true;
}

export default function CodeRunner() {
    const [currentChoice, setCurrentChoice] = React.useState<CurrentChoice>({ language: 'lua', interpreter: 'lua5.4.6' });
    const [vimMode, setVimMode] = React.useState<ReturnType<typeof initVimMode> | null>(null);
    /// Whether the code is currently being executed. Prevents multiple executions.
    const [lastExecution, setLastExecution] = React.useState<ExecutionData>({ pending: false, result: undefined });
    const editorRef = React.useRef<typeof Editor>();

    const langObject = languages[currentChoice.language];
    const currentInterpreter = langObject?.interpreters?.find((i) => i.value === currentChoice.interpreter);
    const currentCompiler = langObject?.compilers?.find((i) => i.value === currentChoice.compiler);

    const timeoutId = React.useRef<NodeJS.Timeout | null>(null);
    const lastExecutionId = React.useRef(0);

    // TODO: Not really working, because it will be block in vim mode and also in insert mode
    editorRef.current?.updateOptions({ tabSize: 4, cursorStyle: vimMode !== null ? "block" : "line" });

    function handleEditorDidMount(editor: typeof Editor, monaco: any) {
        editorRef.current = editor;
        executeCode();
    }

    function debounce() {
        if (timeoutId.current !== null) {
            clearTimeout(timeoutId.current);
        }

        timeoutId.current = setTimeout(() => {
            executeCode();
        }, DEBOUNCE_TIME);
    }

    /// Sends a request to the backend to execute the code.
    /// Returns the response from the backend.
    /// Updates the lastExecution with the response.
    async function executeCode(): Promise<void> {
        const execId = ++lastExecutionId.current;
        const code = editorRef.current!.getValue();
        setLastExecution({ pending: true });
        const response = await fetch('/api/evaluate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ code, currentChoice }),
        })
        if (!response.ok) {
            throw new Error('Failed to execute code');
        }

        // This means that code that was executed after this one was faster
        // and has already finished. Therefore we should ignore the result of this one.
        if (execId !== lastExecutionId.current) {
            return;
        }

        setLastExecution({ pending: false, result: await response.json() });
    }

    if (!langObject) {
        return <div>Language not found</div>;
    }

    return (
        <div className="flex h-full flex-col">
            <main className="flex flex-col grow">
                <div className="grow flex flex-col">
                    <div className="flex flex-row">
                        <Button variant="contained" onClick={executeCode}>Run</Button>
                        <Select
                            value={langObject}
                            getOptionValue={(o) => o.name}
                            options={Object.values(languages)}
                            onChange={(conf) =>
                                setCurrentChoice({
                                    ...currentChoice,
                                    language: conf!.name,
                                    interpreter: conf!.interpreters?.at(0)?.value,
                                    compiler: conf!.compilers?.at(0)?.value,
                                })
                            }
                        />
                        {currentInterpreter && (
                            <Select
                                value={currentInterpreter}
                                options={langObject.interpreters}
                                onChange={(opt) => setCurrentChoice({ ...currentChoice, interpreter: opt!.value })}
                            />
                        )}
                        {currentCompiler && (
                            <Select
                                value={currentCompiler}
                                options={langObject.compilers}
                                onChange={(opt) => setCurrentChoice({ ...currentChoice, compiler: opt!.value })}
                            />
                        )}
                        <CheckBox
                            onChange={async (checked: boolean) => {
                                if (checked) {
                                    setVimMode(initVimMode(editorRef.current!));
                                } else {
                                    vimMode?.dispose();
                                    setVimMode(null);
                                }
                            }}
                            label="Vim mode"
                        />
                        <ShareButton onClick={() => createSharedLink(currentChoice, editorRef.current.value)}/>
                    </div>
                    <div className="grow">
                        <Editor
                            height="100%"
                            defaultLanguage="javascript"
                            defaultValue="print(1)"
                            onMount={handleEditorDidMount}
                            onChange={debounce}
                        />
                    </div>
                </div>
                <CodeOutput executionData={lastExecution} />
            </main>
        </div>
    );
}
