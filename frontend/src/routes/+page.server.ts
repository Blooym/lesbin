import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
    return {
        apiConfig: {
            paste: {
                maxSizeBytes: locals.apiConfig.paste.maxSizeBytes,
                maxExpiry: locals.apiConfig.paste.maxExpiry,
                expiryRequired: locals.apiConfig.paste.expiryRequired
            }
        }
    };
};
