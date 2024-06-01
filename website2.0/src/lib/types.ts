import { Language } from '../lib/languages';

export type CurrentChoice = {
    language: Language;
    interpreter?: string;
    compiler?: string;
};

export type ExecutorResponse = {
    stdout: string;
    stderr: string;
};

export type ExecutionData = {
    pending: boolean;
    result?: ExecutorResponse;
};
