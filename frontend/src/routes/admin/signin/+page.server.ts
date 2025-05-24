import {
    AUTHENTICATION_RETURN_PARAM_NAME,
    AUTHENTICATION_TOKEN_COOKIE_NAME,
    AUTHENTICATION_TOKEN_COOKIE_OPTS
} from '$lib/constants';
import { apiUrl } from '$lib/server/api';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = ({ url, locals }) => {
    const returnUrl = url.searchParams.get(AUTHENTICATION_RETURN_PARAM_NAME);
    if (locals.authenticationToken) {
        return redirect(307, returnUrl ?? '/');
    }
    return {
        returnUrl
    };
};

export const actions: Actions = {
    default: async ({ request, cookies, url, locals, fetch }) => {
        const data = await request.formData();
        const token = data.get('token') as string;
        if (!token || token.trim().length === 0) {
            return fail(400, { error: { message: 'Missing token' } });
        }
        
        const returnUrl = url.searchParams.get(AUTHENTICATION_RETURN_PARAM_NAME) ?? '/';
        const res = await fetch(apiUrl('admin/authenticate'), {
            headers: {
                Authorization: `Bearer ${token}`
            }
        });
        if (!res.ok) {
            return fail(401, { error: { message: 'Invalid token' } });
        }

        // Auth succeeded, store it and redirect.
        cookies.set(AUTHENTICATION_TOKEN_COOKIE_NAME, token, AUTHENTICATION_TOKEN_COOKIE_OPTS);
        locals.authenticationToken = token;
        throw redirect(303, returnUrl);
    }
};
