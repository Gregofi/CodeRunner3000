import type { Reroute } from '@sveltejs/kit';

export const reroute: Reroute = ({ url }) => {
	if (url.pathname.startsWith('/code/s')) {
		return '/code';
	}
};
