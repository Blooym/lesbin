import { env } from '$env/dynamic/private';
import { API_ACCESS_TOKEN_HEADER, AUTHENTICATION_TOKEN_COOKIE_NAME } from '$lib/constants';
import { apiUrl } from '$lib/server/api';
import type { HandleFetch, ServerInit } from '@sveltejs/kit';

export interface APIConfigurationResponse {
    paste: {
        maxSizeBytes: number;
        maxExpiry: number;
        expiryRequired: boolean;
    };
    report: {
        enabled: boolean;
        minLength: number;
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
    // Setup auth if the event is on an admin endpoint.
    if (event.url.pathname.startsWith('/admin') || event.url.pathname.startsWith('/api/admin')) {
        const token = event.cookies.get(AUTHENTICATION_TOKEN_COOKIE_NAME);
        if (token) {
            try {
                const res = await event.fetch(apiUrl('admin/authenticate'), {
                    headers: {
                        Authorization: `Bearer ${token}`
                    }
                });
                if (res.ok) {
                    event.locals.authenticationToken = token;
                }
            } catch (err) {
                console.error('API failed to verify authentication token', err);
            }
        }
    }

    // Get the API configuration for validations.
    {
        const res = await event.fetch(apiUrl('config'));
        const json: APIConfigurationResponse = await res.json();
        event.locals.apiConfig = json;
        console.log('Loaded API configuration', json);
    }

    return resolve(event, {
        preload: ({ type }) => {
            return type === 'font' || type == 'css' || type == 'js';
        }
    });
};
