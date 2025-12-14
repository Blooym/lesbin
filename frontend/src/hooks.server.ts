import { env } from '$env/dynamic/private';
import { apiUrl } from '$lib/api.server';
import { API_ACCESS_TOKEN_HEADER } from '$lib/constants';
import type { ServerInit } from '@sveltejs/kit';

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
    if (!env.LESBIN_API_URL) {
        throw new Error('LESBIN_API_URL missing in environment');
    }
    if (!env.LESBIN_API_ACCESS_TOKEN) {
        throw new Error('LESBIN_API_ACCESS_TOKEN missing in environment');
    }
};

export const handle = async ({ event, resolve }) => {
    // Get the API configuration for validations.
    const res = await event.fetch(apiUrl('instance/config'), {
        headers: {
            [API_ACCESS_TOKEN_HEADER]: env.LESBIN_API_ACCESS_TOKEN
        }
    });
    const json: APIConfigurationResponse = await res.json();
    event.locals.apiConfig = json;
    return resolve(event, {
        preload: ({ type }) => {
            return type === 'font' || type == 'css' || type == 'js';
        }
    });
};
