import { Language } from '../lib/languages';

export type CurrentChoice = {
    name: Language;
    interpreter?: string;
    compiler?: string;
};
