"use client";

import { Header } from './components/header';
import { CodeOutput } from './components/codeOutput';

import Editor from '@monaco-editor/react';

export default function Home() {
  return (
    <div className="flex h-full flex-col">
      <Header />
      <main className="flex flex-col grow">
        <div className="grow flex">
            <div>
                
            </div>
            <div className="grow">
                <Editor
                    height="100%"
                    defaultLanguage="javascript"
                    defaultValue="// some comment"
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
