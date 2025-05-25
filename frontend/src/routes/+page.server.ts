import { HighlighterLanguages, type HighlighterLanguageKey } from '$lib/highlighter';
import hljs from 'highlight.js';
import type { PageServerLoad } from './$types';

type HighlighterLanguage = {
    name: HighlighterLanguageKey;
    alias: string | null;
};

const highlightAliases: HighlighterLanguage[] = [
    {
        name: 'xml',
        alias: 'html'
    }
];

const highlightLanguages: HighlighterLanguage[] = [
    ...hljs
        .listLanguages()
        .filter((lang): lang is HighlighterLanguageKey => lang in HighlighterLanguages)
        .map((lang) => ({
            name: lang,
            alias: null
        })),
    ...highlightAliases
].sort((a, b) =>
    (a.alias ?? a.name).toLowerCase().localeCompare((b.alias ?? b.name).toLowerCase())
);

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
