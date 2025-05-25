import { AUTHENTICATION_RETURN_PARAM_NAME } from '$lib/constants.js';
import { redirect } from '@sveltejs/kit';

export const load = async ({ url, locals, setHeaders }) => {
    setHeaders({
        'cache-control': 'no-store'
    });

    // Don't redirect loop the signin page.
    if (url.pathname === '/admin/signin' && !locals.authenticationToken) {
        return;
    }
    // Prevent double signouts.
    if (url.pathname === '/admin/signout' && !locals.authenticationToken) {
        return redirect(307, '/');
    }
    // If accessing an admin page without authentication, redirect to login with a return param.
    if (!locals.authenticationToken) {
        return redirect(
            307,
            `/admin/signin?${AUTHENTICATION_RETURN_PARAM_NAME}=${encodeURIComponent(url.pathname + url.search)}`
        );
    }

    return {
        authenticationToken: locals.authenticationToken
    };
};
