<script lang="ts">
    import Button from '$lib/components/button/Button.svelte';
    import { toastManager } from '$lib/state/toasts.svelte.js';
    import { writable } from 'svelte/store';

    const { data } = $props();

    let reports = writable(data.reports);

    async function viewPaste(id: string, decryptionKey: string) {
        open(`/paste/${id}#${decryptionKey}`);
    }

    async function deleteReport(id: string) {
        const res = await fetch(`/api/admin/reports/${id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${data.authenticationToken}`
            }
        });

        if (!res.ok) {
            toastManager.createToast(
                `Failed to delete report: HTTP ${res.status} - ${(await res.json()).message}`,
                { variant: 'error' }
            );
            return;
        }
        reports.update((currentReports) => currentReports.filter((report) => report.id !== id));
        toastManager.createToast(`Successfully dismissed report ${id}`, { variant: 'success' });
    }

    async function deletePaste(id: string) {
        const res = await fetch(`/api/admin/pastes/${id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${data.authenticationToken}`
            }
        });
        if (!res.ok) {
            toastManager.createToast(
                `Failed to delete paste: HTTP ${res.status} - ${(await res.json()).message}`,
                { variant: 'error' }
            );
            return;
        }
        reports.update((currentReports) =>
            currentReports.filter((report) => report.pasteId !== id)
        );
        toastManager.createToast(`Successfully deleted paste ${id}`, { variant: 'success' });
    }
</script>

<svelte:head>
    <title>Paste Reports | Lesbin</title>
</svelte:head>

<h1>Paste Reports</h1>

{#if $reports.length > 0}
    <div class="table-container">
        <table>
            <thead>
                <tr>
                    <th>Report</th>
                    <th>Paste</th>
                    <th>Report Reason</th>
                    <th>Created At</th>
                    <th>Report Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each $reports as report (report.id)}
                    <tr>
                        <td>{report.id}</td>
                        <td>{report.pasteId}</td>
                        <td>{report.reason}</td>
                        <td
                            >{report.createdAt.toLocaleString(undefined, {
                                dateStyle: 'medium',
                                timeStyle: 'long'
                            })}</td
                        >
                        <td>
                            <div class="action-container">
                                <Button
                                    variant="primary"
                                    onclick={() => viewPaste(report.pasteId, report.decryptionKey)}
                                    >Open</Button
                                >
                                <Button variant="neutral" onclick={() => deleteReport(report.id)}
                                    >Dismiss</Button
                                >
                                <Button
                                    variant="destructive"
                                    onclick={() => deletePaste(report.pasteId)}>Delete</Button
                                >
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{:else}
    <p style="margin: 0;">Great news, there are no pending reports to show at this time.</p>
{/if}

<style>
    .table-container {
        overflow-x: auto;
    }
    table {
        width: 100%;
        border-collapse: collapse;
    }
    th,
    td {
        padding: 6px;
        text-align: left;
        border: 1px solid var(--col-on-background);
        white-space: normal;
    }
    th:nth-child(3),
    td:nth-child(3) {
        width: 70%;
    }
    th:nth-child(4),
    td:nth-child(4) {
        width: 100%;
    }

    .action-container {
        display: flex;
        gap: 8px;
        row-gap: 4px;
    }

    /* make the table work better on mobile */
    @media (max-width: 768px) {
        .action-container {
            flex-direction: column;
        }

        th:nth-child(1),
        td:nth-child(1) {
            display: none;
        }
        th:nth-child(3),
        td:nth-child(3) {
            display: none;
        }
    }
</style>
