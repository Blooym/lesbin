import { HighlighterLanguages, type HighlighterLanguageKey } from '$lib/highlighter';
import hljs from 'highlight.js';
import type { PageServerLoad } from './$types';

const highlightLanguages = hljs
    .listLanguages()
    .filter((lang): lang is HighlighterLanguageKey => lang in HighlighterLanguages)
    .sort();

export const load: PageServerLoad = async ({ locals }) => {
    return {
        apiConfig: {
            paste: {
                maxSizeBytes: locals.apiConfig.paste.maxSizeBytes,
                maxExpiry: locals.apiConfig.paste.maxExpiry,
                expiryRequired: locals.apiConfig.paste.expiryRequired
            }
        },
        syntaxHighlightLanguages: highlightLanguages
    };
};
