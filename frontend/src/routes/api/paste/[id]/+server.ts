import { apiUrl } from '$lib/server/api';
import { kitFetchWrapper } from '$lib/server/fetch';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const DELETE: RequestHandler = async ({ params, request, fetch }) => {
    const { id } = params;
    const deletionKey = request.headers.get('authorization');
    if (!deletionKey) {
        return json(
            {
                success: false,
                message: 'Missing deletion key'
            },
            {
                status: 400
            }
        );
    }
    await kitFetchWrapper(fetch, apiUrl(`paste/${id}`), {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
            Authorization: deletionKey
        }
    });
    return json({ success: true });
};
