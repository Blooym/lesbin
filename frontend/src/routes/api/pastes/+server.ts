import { apiUrl } from '$lib/server/api';
import { kitFetchWrapper } from '$lib/server/fetch';
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export interface CreatePasteRequest {
    encryptedTitle: string;
    encryptedContent: string;
    syntaxType: string;
    expiresAt: number | null;
}

export interface CreatePasteResponse {
    id: string;
    deletionKey: string;
}

export const POST: RequestHandler = async ({ request, fetch }) => {
    const requestJson: CreatePasteRequest = await request.json();

    const res = await kitFetchWrapper(fetch, apiUrl('pastes'), {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(requestJson)
    });

    try {
        const jsonRes: CreatePasteResponse = await res.json();
        return json({ success: true, ...jsonRes });
    } catch (err) {
        console.log(`Faile to create paste via API: ${err}`);
        return error(500, { message: 'Server responded with an invalid paste format' });
    }
};
