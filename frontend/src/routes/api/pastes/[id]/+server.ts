import { apiUrl } from '$lib/server/api';
import { kitFetchWrapper } from '$lib/server/fetch';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export interface DeletePasteRequest {
    deletionKey: string;
}

export const DELETE: RequestHandler = async ({ params, request, fetch }) => {
    const { id } = params;
    const requestJson: DeletePasteRequest = await request.json();
    await kitFetchWrapper(fetch, apiUrl(`pastes/${id}`), {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(requestJson)
    });
    return json({ success: true });
};
