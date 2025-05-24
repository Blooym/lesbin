<script lang="ts">
    import { goto } from '$app/navigation';
    import Button from '$lib/components/button/Button.svelte';
    import TextButton from '$lib/components/button/TextButton.svelte';
    import Dialog from '$lib/components/Dialog.svelte';
    import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
    import { decryptData, importKey } from '$lib/cryptography.client';
    import { toastManager } from '$lib/state/toasts.svelte';
    import { HighlightAuto, LineNumbers } from 'svelte-highlight';
    import highlighterTheme from 'svelte-highlight/styles/stackoverflow-dark';
    import type { PageProps } from './$types';

    let { data }: PageProps = $props();
    let showDeleteModal = $state(false);
    let showReportModal = $state(false);
    let viewAsRaw = $state(data.paste.viewRaw);
    let hightlighterWrapLines = $state(true);
    let reportReason = $state('');

    async function decryptPaste() {
        const decryptionKey = location.hash.slice(1);
        if (!decryptionKey) {
            throw new Error('No decryption provided in URL fragment');
        }
        const key = await importKey(decryptionKey);
        return {
            title: await decryptData(data.paste.encryptedTitle, key),
            content: await decryptData(data.paste.encryptedContent, key),
            deletionKey: localStorage.getItem(`dk-${data.paste.id}`)
        };
    }

    async function deletePaste(id: string, deletionKey: string) {
        const res = await fetch(`/api/pastes/${data.paste.id}`, {
            body: JSON.stringify({ id, deletionKey }),
            headers: {
                'Content-Type': 'application/json'
            },
            method: 'DELETE'
        });
        if (res.ok) {
            toastManager.createToast('Successfully deleted paste', { variant: 'success' });
            localStorage.removeItem(`k-${id}`);
            await goto('/');
        } else {
            toastManager.createToast(`Failed to delete paste: ${res.status} ${res.statusText}`, {
                variant: 'error'
            });
        }
    }

    async function reportPaste(id: string, decryptionKey: string, reason: string) {
        const res = await fetch(`/api/pastes/${id}/report`, {
            body: JSON.stringify({
                reason,
                decryptionKey
            }),
            headers: {
                'Content-Type': 'application/json'
            },
            method: 'POST'
        });
        if (!res.ok) {
            toastManager.createToast(`Failed to report paste: ${res.status} ${res.statusText}`, {
                variant: 'error'
            });
        } else {
            toastManager.createToast('Successfully created paste report', { variant: 'success' });
        }
    }

    async function copyToClipboard(content: string, alertOnSuccess: boolean) {
        await navigator.clipboard.writeText(content);
        if (alertOnSuccess) {
            toastManager.createToast('Copied to clipboard', { duration: 1500 });
        }
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
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html highlighterTheme}
</svelte:head>

{#await decryptPaste()}
    <div style="display: flex; align-items: center; gap: 8px; margin: 0; padding: 0;">
        <h1>Loading Paste</h1>
        <LoadingSpinner />
    </div>
    <p>
        This may take some time depending on the speed of your device and the size of the paste as
        content has to be decrypted and syntax highlighted by your browser. Please note that this
        browser tab may become temporarily unresponsive whilst this process takes place.
    </p>
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
        <Dialog title="Report Paste" bind:showModal={showReportModal} closeText="Cancel">
            <p>Please specify your reason for reporting this paste</p>
            <textarea
                style="width: 100%; padding: 8px; height: 10rem; resize: none;"
                bind:value={reportReason}
                placeholder="For example: 'this paste contains offensive content'"
            ></textarea>
            <small>
                By reporting this paste you will send the decryption key to the server
                administrators so they can review your report.
            </small>

            {#snippet actions()}
                <Button
                    onclick={() => {
                        reportPaste(data.paste.id, window.location.hash.slice(1), reportReason);
                        showReportModal = false;
                        reportReason = '';
                    }}
                    disabled={reportReason.trim().length < 10}
                    variant="destructive">Report</Button
                >
            {/snippet}
        </Dialog>
    </div>

    <div class="paste-info">
        <h1>{decryptedPaste.title}</h1>
        <small>
            Created
            <time datetime={new Date(data.paste.createdAt).toISOString()}>
                {new Date(data.paste.createdAt).toLocaleString(undefined, {
                    dateStyle: 'medium',
                    timeStyle: 'long'
                })}
            </time>
            {#if data.paste.expiresAt}
                - Expires
                <time datetime={new Date(data.paste.expiresAt).toISOString()}>
                    {new Date(data.paste.expiresAt).toLocaleString(undefined, {
                        dateStyle: 'medium',
                        timeStyle: 'long'
                    })}
                </time>
            {/if}
        </small>
    </div>
    <div class="paste-content-container">
        {#if !viewAsRaw}
            <HighlightAuto langtag code={decryptedPaste.content} let:highlighted>
                <LineNumbers
                    highlightedLines={data.paste.highlightedLines}
                    wrapLines={hightlighterWrapLines}
                    {highlighted}
                />
            </HighlightAuto>
        {:else}
            <textarea id="pasteContentRaw" spellcheck="false" autocomplete="off" readonly
                >{decryptedPaste.content}</textarea
            >
        {/if}
    </div>
    <div class="paste-actions">
        <div>
            <input id="viewRawCheckbox" bind:checked={viewAsRaw} type="checkbox" />
            <label for="viewRawCheckbox">View Raw</label>
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
                    onclick={() => copyToClipboard(decryptedPaste.content, true)}>Copy</TextButton
                > :

                {#if decryptedPaste.deletionKey}
                    <TextButton variant="destructive" onclick={() => (showDeleteModal = true)}
                        >Delete</TextButton
                    >
                {:else}
                    <TextButton variant="destructive" onclick={() => (showReportModal = true)}
                        >Report</TextButton
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

    /* Pastes */
    .paste-content-container {
        height: 75dvh;
        border: 1px dashed var(--col-primary);
        border-radius: var(--rounding-normal);
        background-color: var(--col-formfield-background);
        color: var(--col-on-formfield-background);
        overflow: auto;
        margin: 4px 0;
        display: flex;

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
    }

    /* Errors */
    .error-container {
        color: red;
        text-align: center;
        padding: 1rem;
        border: 1px solid red;
    }
</style>
