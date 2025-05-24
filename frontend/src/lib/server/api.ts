import { env } from '$env/dynamic/private';

/**
 * Create a URL to the specified API resource.
 * @param path The path to the backend resource with no beginning or end `/`.
 * @param queryParams The query parameters to include alongside the request.s
 * @returns A fully-formed URL for API resource.
 */
export function apiUrl(
    path: string,
    queryParams?: Record<string, string>
) {
    const url = new URL(path, env.API_URL);
    if (queryParams) {
        url.search = new URLSearchParams(queryParams).toString(); 
    }
    return url;
}
