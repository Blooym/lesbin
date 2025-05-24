import { apiUrl } from '$lib/server/api';
import type { PageServerLoad } from './$types';

interface Report {
    id: string;
    pasteId: string;
    decryptionKey: string;
    reason: string;
}

interface APIReport extends Report {
    createdAt: number;
}

export interface FrontendReport extends Report {
    createdAt: Date;
}

export const load: PageServerLoad = async ({ fetch, locals }) => {
    const response = await fetch(
        apiUrl(`admin/reports`, {
            page: '1',
            perPage: '250'
        }),
        {
            headers: {
                Authorization: `Bearer ${locals.authenticationToken}`
            }
        }
    );
    const json = await response.json();
    const reports = json.reports.map((report: APIReport) => ({
        id: report.id,
        pasteId: report.pasteId,
        decryptionKey: report.decryptionKey,
        reason: report.reason,
        createdAt: new Date(report.createdAt * 1000)
    })) as FrontendReport[];

    return {
        reports
    };
};
