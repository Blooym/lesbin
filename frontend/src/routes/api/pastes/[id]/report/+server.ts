import { apiUrl } from '$lib/server/api';
import { kitFetchWrapper } from '$lib/server/fetch';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export interface ReportPasteRequest {
    decryptionKey: string;
    reason: string;
}

export const POST: RequestHandler = async ({ params, request, fetch }) => {
    const { id } = params;
    const requestJson: ReportPasteRequest = await request.json();
    requestJson.reason = requestJson.reason.trim();
    await kitFetchWrapper(fetch, apiUrl(`pastes/${id}/report`), {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(requestJson)
    });
    return json({ success: true });
};
