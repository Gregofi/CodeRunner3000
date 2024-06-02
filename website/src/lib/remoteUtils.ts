import type { LangKey, Result } from '$lib/types';

export const sendCodeToServer = async (
	code: string,
	language: LangKey,
	compiler?: string,
	executor?: string
): Promise<Result> => {
	const body = JSON.stringify({
		code,
		language,
		compiler,
		executor
	});
	const response = await fetch('/api/code-eval', {
		method: 'POST',
		body,
		mode: 'cors',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		}
	});
	if (response.ok) {
		return await response.json();
	} else {
		throw new Error('Could not evaluate');
	}
};
