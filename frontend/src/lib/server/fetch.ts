import { error } from '@sveltejs/kit';

/**
 * A wrapper for `fetch` that wraps any request failures using `@sveltejs/kit`'s `error` type for nicer frontend formatting.
 *
 * This method will use a generic message for network errors and either the text response or statusText for codes not in the `2xx` range.
 *
 * @param fetchFn The fetch function to use.
 * @param input The source to make the request to.
 * @param init Optional request parameters.
 * @returns The response if successful. Successful is determined by a status code in the `2xx` range.
 */
export async function kitFetchWrapper(
    fetchFn: typeof fetch,
    input: Parameters<typeof fetchFn>[0],
    init?: Parameters<typeof fetchFn>[1]
) {
    let response: Response;
    try {
        response = await fetchFn(input, init);
    } catch (err) {
        console.error('Failed to send request:', err);
        throw error(500, { message: 'An internal network error occured' });
    }
    if (!response.ok) {
        let message = response.statusText;
        try {
            const text = await response.text();
            if (text.trim()) message = text;
        } catch (err) {
            console.error('Failed to parse error response:', err);
        }
        throw error(response.status, { message });
    }
    return response;
}
