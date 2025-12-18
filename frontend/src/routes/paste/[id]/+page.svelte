<script lang="ts">
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';
    import Button from '$lib/components/button/Button.svelte';
    import TextButton from '$lib/components/button/TextButton.svelte';
    import Dialog from '$lib/components/Dialog.svelte';
    import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
    import { decryptData, importKey } from '$lib/cryptography';
    import { deletePaste as deletePasteRemote } from '$lib/functions/paste.remote';
    import { HighlighterLanguages, type HighlighterLanguageKey } from '$lib/highlighter';
    import { toastManager } from '$lib/state/toasts.svelte';
    import { Highlight, LineNumbers } from 'svelte-highlight';
    import HighlightThemeDark from 'svelte-highlight/styles/tokyo-night-dark';
    import HighlightThemeLight from 'svelte-highlight/styles/xcode';
    import type { PageProps } from './$types';

    const { data }: PageProps = $props();
    let showDeleteModal = $state(false);
    let viewAsRaw = $derived(data.paste.viewRaw);
    let hightlighterWrapLines = $state(true);
    let isDarkTheme = $state(true);

    $effect(() => {
        const darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
        isDarkTheme = darkModeQuery.matches;
        const handler = (e: MediaQueryListEvent) => {
            isDarkTheme = e.matches;
        };
        darkModeQuery.addEventListener('change', handler);
        return () => darkModeQuery.removeEventListener('change', handler);
    });

    async function decryptPaste() {
        const decryptionKey = location.hash.slice(1).trim();
        if (!decryptionKey) {
            throw new Error('No decryption key provided in URL fragment');
        }
        const key = await importKey(decryptionKey);
        const syntaxType = await decryptData(data.paste.encryptedSyntaxType, key);
        return {
            title: await decryptData(data.paste.encryptedTitle, key),
            content: await decryptData(data.paste.encryptedContent, key),
            deletionKey: localStorage.getItem(`dk-${data.paste.id}`),
            syntaxType:
                syntaxType in HighlighterLanguages
                    ? (syntaxType as HighlighterLanguageKey)
                    : ('plaintext' as HighlighterLanguageKey)
        };
    }

    async function deletePaste(id: string, deletionKey: string) {
        deletionKey = deletionKey.trim();
        const res = await deletePasteRemote({ id, key: deletionKey });
        if (res.success) {
            toastManager.createToast('Successfully deleted paste', { variant: 'success' });
            localStorage.removeItem(`dk-${id}`);
            await goto(resolve('/'));
        } else {
            toastManager.createToast(`Failed to delete paste: ${res.message}`, {
                variant: 'error'
            });
        }
    }

    async function copyToClipboard(content: string) {
        await navigator.clipboard.writeText(content);
        toastManager.createToast('Copied to clipboard', { duration: 1500 });
    }

    async function saveToFile(title: string, content: string) {
        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = title;
        a.click();
        URL.revokeObjectURL(url);
        a.remove();
    }
</script>

<svelte:head>
    <title>View Paste | Lesbin</title>
    <meta name="title" content="View Paste" />
    <meta
        name="description"
        content="Open this page in your browser to view the contents of this paste"
    />
    <meta name="robots" content="noindex, nofollow" />

    <!-- SAFETY: reviewed & fairly trusted package-->
    {#if isDarkTheme}
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html HighlightThemeDark}
    {:else}
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html HighlightThemeLight}
    {/if}
</svelte:head>

{#await decryptPaste()}
    <div>
        <div class="loading-paste">
            <h1>Loading Paste</h1>
            <LoadingSpinner />
        </div>
        <p>
            This may take some time depending on the speed of your device and the size of the paste
            as content has to be decrypted and syntax highlighted by your browser. Please note that
            this browser tab may become temporarily unresponsive whilst this process takes place.
        </p>
    </div>
{:then decryptedPaste}
    <div>
        <Dialog title="Delete Paste" bind:showModal={showDeleteModal} closeText="Cancel">
            <p>
                Are you sure you want to delete "{decryptedPaste.title}"?
            </p>

            {#snippet actions()}
                <Button
                    onclick={() => {
                        deletePaste(data.paste.id, decryptedPaste.deletionKey!);
                        showDeleteModal = false;
                    }}
                    variant="destructive">Delete</Button
                >
            {/snippet}
        </Dialog>
    </div>
    <div>
        <h1>{decryptedPaste.title}</h1>
        <p>
            Created
            <time datetime={new Date(data.paste.createdAt).toISOString()}>
                {new Intl.DateTimeFormat(undefined, {
                    dateStyle: 'short',
                    timeStyle: 'short'
                }).format(new Date(data.paste.createdAt))}
            </time>
            {#if data.paste.expiresAt}
                / Expires
                <time datetime={new Date(data.paste.expiresAt).toISOString()}>
                    {new Intl.DateTimeFormat(undefined, {
                        dateStyle: 'short',
                        timeStyle: 'short'
                    }).format(new Date(data.paste.expiresAt))}
                </time>
            {/if}
        </p>
    </div>
    <div class="paste-content-container">
        {#if !viewAsRaw}
            <Highlight
                language={HighlighterLanguages[decryptedPaste.syntaxType]}
                code={decryptedPaste.content}
                langtag
                let:highlighted
            >
                <LineNumbers
                    highlightedLines={data.paste.highlightedLines}
                    wrapLines={hightlighterWrapLines}
                    {highlighted}
                />
            </Highlight>
        {:else}
            <textarea id="pasteContentRaw" spellcheck="false" autocomplete="off" readonly
                >{decryptedPaste.content}</textarea
            >
        {/if}
    </div>
    <div class="paste-actions">
        <div>
            <input id="viewRawCheckbox" bind:checked={viewAsRaw} type="checkbox" />
            <label for="viewRawCheckbox">Raw</label>
            {#if !viewAsRaw}
                <input id="lineWrapCheckbox" bind:checked={hightlighterWrapLines} type="checkbox" />
                <label for="lineWrapCheckbox">Line Wrap</label>
            {/if}
        </div>
        <div>
            <small>
                [
                <TextButton
                    variant="neutral"
                    onclick={() => copyToClipboard(decryptedPaste.content)}>Copy</TextButton
                > :
                <TextButton
                    variant="neutral"
                    onclick={() => saveToFile(decryptedPaste.title, decryptedPaste.content)}
                    >Save</TextButton
                >
                {#if decryptedPaste.deletionKey}
                    :
                    <TextButton variant="destructive" onclick={() => (showDeleteModal = true)}
                        >Delete</TextButton
                    >
                {:else if data.report.email}
                    :
                    <a
                        id="reportButton"
                        href={`mailto:${data.report.email}?subject=${encodeURIComponent(`Lesbin Paste Report: ${data.paste.id}`)}&body=${encodeURIComponent(`Hello, I'd like to report this paste on your lesbin instance: <a href='${location.href}'>${location.href}</a>.\n\nMy reason for reporting is: [WRITE YOUR REPORT REASON HERE]\n\n<small><strong>Important Notice</strong>\nReporting encrypted content requires sharing the decryption key. By submitting this report, you:\n- Grant the instance moderators access to decrypt and review the content\n- Understand that the key will be transmitted through email and may be stored by any involved email service providers\n- Understand that your report will be handled according to the instance's policies, and you may not receive a direct response about it.\n- Abuse of the report system may lead to you being restricted entirely.</small>`)}`}
                        >Report</a
                    >
                {/if}
                ]
            </small>
        </div>
    </div>
{:catch error}
    <div class="error-container">
        <h1>Decryption Failure</h1>
        <p>
            The contents of this paste could not be decrypted - please make sure the link and
            decryption key are correct or consult the error below for more information.
        </p>
        <small>Info: {error}</small>
    </div>
{/await}

<style>
    h1 {
        margin: 0.4rem 0;
    }
    p {
        margin: 0.2rem 0;
    }
    .loading-paste {
        display: flex;
        align-items: center;
        gap: 8px;
        margin: 0;
        padding: 0;
    }
    .paste-content-container {
        height: 75vh;
        border: 1px dashed var(--accent-colour);
        border-radius: var(--rounding-normal);
        background-color: var(--mantle-colour);
        color: var(--text-colour);
        overflow: auto;
        margin: 4px 0;
        display: flex;
        flex-direction: column;

        #pasteContentRaw {
            background-color: inherit;
            color: inherit;
            flex: 1;
            width: 100%;
            padding: 10px;
            border: unset;
            resize: none;
        }
        #pasteContentRaw:focus {
            outline: unset;
            box-shadow: unset;
        }
    }
    .paste-actions {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-wrap: wrap;
        #reportButton {
            color: var(--negative-colour);
            &:hover {
                color: color-mix(in srgb, var(--negative-colour) 70%, white);
            }
        }
    }
    .error-container {
        color: var(--negative-colour);
        text-align: center;
        padding: 1rem;
        border: 1px solid var(--negative-colour);
    }
</style>
