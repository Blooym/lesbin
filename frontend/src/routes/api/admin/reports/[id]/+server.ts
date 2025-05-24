import { apiUrl } from '$lib/server/api';
import { kitFetchWrapper } from '$lib/server/fetch';
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const DELETE: RequestHandler = async ({ params, locals, fetch }) => {
    if (!locals.authenticationToken) {
        return error(401);
    }
    const { id } = params;
    await kitFetchWrapper(fetch, apiUrl(`admin/reports/${id}`), {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${locals.authenticationToken}`
        }
    });
    return json({ success: true });
};
