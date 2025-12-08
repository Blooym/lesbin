<script lang="ts">
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';
    import Button from '$lib/components/button/Button.svelte';
    import TextButton from '$lib/components/button/TextButton.svelte';
    import { encryptData, exportKey, generateKey } from '$lib/cryptography.client';
    import type { HighlighterLanguageKey } from '$lib/highlighter';
    import { toastManager } from '$lib/state/toasts.svelte';
    import { onMount } from 'svelte';
    import type { CreatePasteRequest, CreatePasteResponse } from './api/pastes/+server';

    const { data } = $props();
    const refreshDatesInterval = 5000;

    let pasteTitle: string = $state('');
    let pasteContent: string = $state('');
    let pasteSyntaxType: HighlighterLanguageKey = $state('plaintext');
    let pasteExpiry: string | undefined = $state(undefined);
    let minPasteExpiry: string = $state(getMinimumPasteExpiry());
    let maxPasteExpiry: string = $state(getMaximumPasteExpiry());
    let creatingPaste: boolean = $state(false);

    onMount(() => {
        const minInterval = setInterval(
            () => (minPasteExpiry = getMinimumPasteExpiry()),
            refreshDatesInterval
        );
        const maxInterval = setInterval(
            () => (maxPasteExpiry = getMaximumPasteExpiry()),
            refreshDatesInterval
        );
        return () => {
            clearInterval(minInterval);
            clearInterval(maxInterval);
        };
    });

    /**
     * Get the minimum allowed paste expiry as a local timezone date string.
     */
    function getMinimumPasteExpiry(): string {
        return formatDateToInputValue(new Date(Date.now() + 2 * 1000));
    }

    /**
     * Get the maximum allowed paste expiry as a local timezone date string.
     */
    function getMaximumPasteExpiry(): string {
        return formatDateToInputValue(new Date(Date.now() + data.apiConfig.paste.maxExpiry * 1000));
    }

    /**
     * Formats the given date into a local timezone string compatiable with input fields.
     * @param date The date to format.
     */
    function formatDateToInputValue(date: Date): string {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        const hours = String(date.getHours()).padStart(2, '0');
        const minutes = String(date.getMinutes()).padStart(2, '0');
        return `${year}-${month}-${day}T${hours}:${minutes}`;
    }

    /**
     * Fills in the create paste form fields using data from the selected file.
     *
     * The selected file must be valid UTF-8 and be below the maximum size allowed by the server.
     */
    function fillFieldsFromFile() {
        const input = document.createElement('input');
        input.type = 'file';
        input.onchange = (e) => {
            const selectedFile = (e.target as HTMLInputElement).files?.[0];
            if (!selectedFile) {
                return;
            }

            if (selectedFile.size > data.apiConfig.paste.maxSizeBytes) {
                toastManager.createToast(
                    `File size exceeds the maximum size allowed by the server (${selectedFile.size}bytes > ${data.apiConfig.paste.maxSizeBytes}bytes)`,
                    { variant: 'error', duration: 4000 }
                );
                return;
            }

            const reader = new FileReader();
            reader.onload = (e) => {
                const buffer = e.target?.result as ArrayBuffer;
                try {
                    const decoder = new TextDecoder('utf-8', { fatal: true });
                    const content = decoder.decode(buffer);
                    // Set the paste fields based on the file content.
                    pasteTitle = selectedFile.name.trim();
                    pasteContent = content.trim();
                    // Make a best-effort attempt to detect the syntax type from MIME type.
                    const mimeType = selectedFile.type.split('/')[1]?.replace(/^x-/, '') || '';
                    pasteSyntaxType = data.syntaxHighlightLanguages.some(
                        (lang) => lang.name === mimeType || lang.alias === mimeType
                    )
                        ? (mimeType as HighlighterLanguageKey)
                        : 'plaintext';
                } catch {
                    toastManager.createToast(
                        'Failed to load file content - it must be valid UTF-8 text.',
                        {
                            variant: 'error',
                            duration: 4000
                        }
                    );
                }
            };
            reader.readAsArrayBuffer(selectedFile);
        };
        input.click();
        input.remove();
    }

    /**
     * Create a new paste using the given component state.
     */
    async function createPaste(event: Event) {
        try {
            event.preventDefault();
            creatingPaste = true;

            // Create a new encryption key.
            const key = await generateKey();
            const exportedKey = await exportKey(key);

            // Format and encrypt the paste and send it to the server.
            const pasteRequest: CreatePasteRequest = {
                encryptedTitle: await encryptData(pasteTitle.trim(), key),
                encryptedContent: await encryptData(pasteContent.trim(), key),
                encryptedSyntaxType: await encryptData(pasteSyntaxType, key),
                expiresAt: pasteExpiry ? Math.floor(new Date(pasteExpiry).getTime() / 1000) : null
            };
            const pasteSize =
                pasteRequest.encryptedTitle.length + pasteRequest.encryptedContent.length;

            // Don't try to upload pastes that are above the max size.
            if (pasteSize > data.apiConfig.paste.maxSizeBytes) {
                toastManager.createToast(
                    `Paste size exceeds the maximum size allowed by the server (${pasteSize}bytes > ${data.apiConfig.paste.maxSizeBytes}bytes)`,
                    { variant: 'error', duration: 7000 }
                );
                return;
            }

            const response = await fetch('/api/paste', {
                body: JSON.stringify(pasteRequest),
                headers: {
                    'Content-Type': 'application/json'
                },
                method: 'POST'
            });
            if (!response.ok) {
                try {
                    const json = await response.json();
                    toastManager.createToast(
                        `Failed to create paste: ${json.message} (${response.statusText})`,
                        { variant: 'error' }
                    );
                } catch {
                    toastManager.createToast(
                        `Failed to create paste:  ${response.status} ${response.statusText}`,
                        { variant: 'error' }
                    );
                }
                return;
            }
            const json: CreatePasteResponse = await response.json();
            window.localStorage.setItem(`dk-${json.id}`, json.deletionKey);
            toastManager.createToast(`Successfully created paste "${pasteTitle}"`, {
                variant: 'success'
            });
            await goto(resolve(`/paste/${json.id}#${exportedKey}`));
        } catch (error) {
            console.error(error);
            toastManager.createToast(`Failed to create paste ${error}`, { variant: 'error' });
        } finally {
            creatingPaste = false;
        }
    }
</script>

<svelte:head>
    <title>New Paste | Lesbin</title>
    <meta name="title" content="New Paste" />
    <meta
        name="description"
        content="Share your dreams, fanfiction, logs and code. Create a new end-to-end encrypted paste on lesbin for free."
    />
</svelte:head>

<h1>New Paste</h1>
<p>
    Pastes are end-to-end encrypted and are only accessible to those with the link - <a
        href="https://codeberg.org/Blooym/lesbin#how-data-is-stored-encrypted">learn more</a
    >.
</p>

<form id="pasteForm" onsubmit={createPaste}>
    <label for="pasteTitle">Title</label>
    <input
        id="pasteTitle"
        type="text"
        placeholder="code.rs"
        bind:value={pasteTitle}
        maxlength={64}
        minlength={3}
        required
        autocomplete="off"
    />
    <label for="pasteSyntax">Syntax Highlighting</label>
    <select required bind:value={pasteSyntaxType} id="pasteSyntax">
        {#each data.syntaxHighlightLanguages as syntaxLanguage (syntaxLanguage.alias ?? syntaxLanguage.name)}
            <option selected={syntaxLanguage.name === 'plaintext'} value={syntaxLanguage.name}
                >{syntaxLanguage.alias ?? syntaxLanguage.name}</option
            >
        {/each}
    </select>
    <label for="pasteExpiry"
        >Expiry - local time {#if !data.apiConfig.paste.expiryRequired}&nbsp;(Optional){/if}</label
    >
    <input
        id="pasteExpiry"
        bind:value={pasteExpiry}
        required={data.apiConfig.paste.expiryRequired}
        type="datetime-local"
        min={minPasteExpiry}
        max={maxPasteExpiry}
        autocomplete="off"
    />
    <label for="pasteContent">Content</label>
    <textarea
        id="pasteContent"
        bind:value={pasteContent}
        placeholder="println!(&quot;this is my paste&quot;);"
        required
        minlength={8}
        autocomplete="off"
        spellcheck="false"
    ></textarea>
    <div>
        <TextButton type="button" variant="primary" onclick={fillFieldsFromFile}
            >Upload file</TextButton
        >
    </div>
    <Button
        style="width: 15rem; padding: 10px; margin: 15px auto 0 auto;"
        variant="primary"
        type="submit"
        disabled={creatingPaste}>Create</Button
    >
</form>

<style>
    h1 {
        margin: 0.4rem 0;
    }
    p {
        margin: 0.2rem 0;
    }
    #pasteForm {
        display: flex;
        flex-direction: column;
        margin-top: 6px;
        flex: auto;
        label {
            margin: 0.8rem 0;
            font-weight: bold;
        }
        textarea {
            min-height: 15rem;
            flex: auto;
            resize: none;
        }
    }
</style>
