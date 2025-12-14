import { command } from '$app/server';
import { env } from '$env/dynamic/private';
import { apiUrl } from '$lib/api.server';
import { API_ACCESS_TOKEN_HEADER } from '$lib/constants';
import * as v from 'valibot';

export const createPaste = command(
    v.object({
        encryptedTitle: v.pipe(v.string(), v.nonEmpty()),
        encryptedContent: v.pipe(v.string(), v.nonEmpty()),
        encryptedSyntaxType: v.pipe(v.string(), v.nonEmpty()),
        expiresAt: v.nullable(v.number())
    }),
    async ({ encryptedTitle, encryptedContent, encryptedSyntaxType, expiresAt }) => {
        try {
            const response = await fetch(apiUrl('paste'), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    [API_ACCESS_TOKEN_HEADER]: env.LESBIN_API_ACCESS_TOKEN
                },
                body: JSON.stringify({
                    encryptedTitle,
                    encryptedContent,
                    encryptedSyntaxType,
                    expiresAt
                })
            });

            if (!response.ok) {
                console.error(
                    `Upstream server responded with an unsuccessful status code: ${response.statusText} (${response.status})`
                );
                return {
                    success: false,
                    message: 'An internal error occured whilst creating a paste'
                };
            }

            const json: {
                id: string;
                deletionKey: string;
            } = await response.json();
            return {
                success: true,
                id: json.id,
                deletionKey: json.deletionKey
            };
        } catch (err) {
            console.error(`Failed to create paste: ${err}`);
            return {
                success: false,
                message: 'An internal error occured whilst creating a paste'
            };
        }
    }
);

export const deletePaste = command(
    v.object({
        id: v.pipe(v.string(), v.nonEmpty()),
        key: v.pipe(v.string(), v.nonEmpty())
    }),
    async ({ id, key }) => {
        try {
            const res = await fetch(apiUrl(`paste/${id}`), {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${key}`,
                    [API_ACCESS_TOKEN_HEADER]: env.LESBIN_API_ACCESS_TOKEN
                }
            });

            if (!res.ok) {
                return {
                    success: false,
                    message: 'An internal error occured whilst deleting thos paste'
                };
            }

            return { success: true };
        } catch (e) {
            console.error(e);
            return {
                success: false,
                message: 'An internal error occured whilst deleting this paste'
            };
        }
    }
);
