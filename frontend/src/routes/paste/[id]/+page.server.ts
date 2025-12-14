import { LESBIN_API_ACCESS_TOKEN } from '$env/static/private';
import { apiUrl } from '$lib/api.server';
import { API_ACCESS_TOKEN_HEADER } from '$lib/constants';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

const HIGHLIGHT_LINES_QUERY_PARAM = 'lines';

export const load: PageServerLoad = async ({ params, url, fetch, locals }) => {
    const { id } = params;

    // Fetch paste data.
    let response: Response;
    try {
        response = await fetch(apiUrl(`paste/${id}`), {
            headers: {
                'Content-Type': 'application/json',
                [API_ACCESS_TOKEN_HEADER]: LESBIN_API_ACCESS_TOKEN
            }
        });

        if (!response.ok) {
            console.error(
                'Get paste response did not return as successful',
                response.status,
                response.statusText
            );
            return error(500, 'An internal error occured while loading this paste.');
        }

        if (response.status === 404) {
            return error(404, 'This paste does not exist.');
        }
    } catch (err) {
        console.log('Get paste request failed', err);
        error(500, 'An internal error occured while loading this paste');
    }

    // Support for line highlighting.
    const highlightParam = url.searchParams.get(HIGHLIGHT_LINES_QUERY_PARAM);
    let highlight: number[] = [];
    if (highlightParam) {
        const [a, b] = highlightParam.split(',').map(Number);
        if (!isNaN(a) && !isNaN(b)) {
            const start = Math.min(a, b) - 1;
            const end = Math.max(a, b) - 1;
            highlight = Array.from({ length: end - start + 1 }, (_, i) => start + i);
        } else if (!isNaN(a)) {
            highlight = [a - 1];
        }
    }

    // Send data back to client
    try {
        const json = await response.json();
        return {
            paste: {
                id,
                encryptedTitle: json.encryptedTitle,
                encryptedContent: json.encryptedContent,
                encryptedSyntaxType: json.encryptedSyntaxType,
                createdAt: json.createdAt * 1000,
                expiresAt: json.expiresAt ? json.expiresAt * 1000 : null,
                highlightedLines: highlight,
                viewRaw: url.searchParams.get('raw')?.toLowerCase() === 'true'
            },
            report: {
                email: locals.apiConfig.report.email
            }
        };
    } catch (err) {
        console.error('JSON response was malformed', err);
        return error(500, 'An internal error occured while loading this paste.');
    }
};
