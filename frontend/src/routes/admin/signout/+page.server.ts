import { AUTHENTICATION_TOKEN_COOKIE_NAME, AUTHENTICATION_TOKEN_COOKIE_OPTS } from '$lib/constants';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = ({ cookies }) => {
    cookies.delete(AUTHENTICATION_TOKEN_COOKIE_NAME, AUTHENTICATION_TOKEN_COOKIE_OPTS);
    return redirect(307, '/');
};
