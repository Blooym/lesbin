import { env } from '$env/dynamic/private';
import { API_ACCESS_TOKEN_HEADER } from '$lib/constants';
import { apiUrl } from '$lib/server/api';
import type { HandleFetch, ServerInit } from '@sveltejs/kit';

export interface APIConfigurationResponse {
    paste: {
        maxSizeBytes: number;
        maxExpiry: number;
        expiryRequired: boolean;
    };
    report: {
        email: string | null;
    };
}

export const init: ServerInit = async () => {
    // https://github.com/sveltejs/kit/issues/14347
    if (env.npm_lifecycle_event === 'build') {
        console.log('Skipping init during build / prerendering');
        return;
    }

    if (!env.LESBIN_API_URL) {
        throw new Error('LESBIN_API_URL missing in environment');
    }
    if (!env.LESBIN_API_ACCESS_TOKEN) {
        throw new Error('LESBIN_API_ACCESS_TOKEN missing in environment');
    }
};

export const handleFetch: HandleFetch = async ({ request, fetch }) => {
    // Attach access token to API requests.
    if (new URL(request.url).origin == new URL(env.LESBIN_API_URL).origin) {
        request.headers.set(API_ACCESS_TOKEN_HEADER, env.LESBIN_API_ACCESS_TOKEN);
    }
    return fetch(request);
};

export const handle = async ({ event, resolve }) => {
    // Get the API configuration for validations.
    const res = await event.fetch(apiUrl('instance/config'));
    const json: APIConfigurationResponse = await res.json();
    event.locals.apiConfig = json;

    return resolve(event, {
        preload: ({ type }) => {
            return type === 'font' || type == 'css' || type == 'js';
        }
    });
};
