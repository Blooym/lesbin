import * as highlighterLanguages from 'svelte-highlight/languages';

export type HighlighterLanguageKey = keyof typeof highlighterLanguages;
export const HighlighterLanguages = highlighterLanguages;
