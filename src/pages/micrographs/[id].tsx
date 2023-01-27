import { useRouter } from 'next/router'
import ImageViewer from '../../components/ImageViewer';

import useSWR from 'swr'
import { invoke } from '@tauri-apps/api/tauri';
import type { Micrograph } from '../../../src-tauri/bindings/Micrograph';
import type { Segment } from "../../../src-tauri/bindings/Segment";
import Loading from '../../components/layout/Loading';
import { ProgressBar } from '@tremor/react';

const MicrographPage = () => {
    const router = useRouter();
    const { id } = router.query;

    const { data: micrograph, error: micrographError, isLoading: micrographIsLoading } = useSWR(`/micrographs/${id}`, () => micrographFetcher(String(id)), {
        refreshInterval(latestData) {
            // Refresh every second if the status is "new"
            if (latestData?.status === "new") return 1000;
            return 0;
        },
    });

    const { data: segments, error: segmentsError, isLoading: segmentsIsLoading } = useSWR(`/micrographs/${id}/segments`, () => segmentsFetcher(String(id)), {
        refreshInterval(latestData) {
            // Refresh every second if no segments are available
            if (latestData && latestData.length == 0) return 1000;
            // Refresh every second if not all segments are processed
            if (latestData && countProcessed(latestData) < latestData.length) return 1000;
            return 0;
        },
    });

    // count the number of segments that have status ok
    const processedSegments = segments && countProcessed(segments);

    const processedPercentage = processedSegments && segments ? processedSegments / segments.length * 100 : 0;

    if (micrographError) return <div>Es ist ein Fehler aufgetreten: {micrographError}</div>
    if (micrographIsLoading) return <Loading />

    return (
        <div className="space-y-4">
            <a onClick={() => router.back()} className="text-gray-600 hover:text-gray-900 cursor-pointer">← Zurück</a>
            <div className="bg-white shadow sm:rounded-lg">
                <div className="px-4 py-5 sm:px-6">
                    <h3 className="text-lg font-medium leading-6 text-gray-900"><>{micrograph.name} (ID: {micrograph.uuid})</></h3>
                </div>
            </div>
            {micrograph && (
                <div className="bg-white shadow sm:rounded-lg p-4 space-y-4">
                    <h3 className="text-lg font-medium leading-6 text-gray-900">Mikroskopaufnahme</h3>
                    <ImageViewer micrograph={micrograph} segments={segments} />
                </div>
            )}
            {micrograph.status !== "new" && segments && segments.length > 0 && (
                <div className="bg-white shadow sm:rounded-lg p-4 space-y-4">
                    <h3 className="text-lg font-medium leading-6 text-gray-900">Analyse</h3>

                    {processedSegments !== segments.length && <ProgressBar showAnimation percentageValue={processedPercentage} label={`${processedSegments}/${segments.length}`} />}
                </div>
            )}
        </div>
    );
}

const micrographFetcher = (id: string) => {
    if (!id) return Promise.reject("No id provided");
    return invoke("get_micrograph", { micrographId: id }).then((res: string) => JSON.parse(res) as Micrograph);
};
const segmentsFetcher = (id: string) => {
    if (!id) return Promise.reject("No id provided");
    return invoke("get_segments", { micrographId: id }).then((res: string) => JSON.parse(res) as Segment[])
};

const countProcessed = (segments: Segment[]) => {
    return segments.reduce((acc, segment) => {
        if (segment.status === "ok") return acc + 1;
        return acc;
    }, 0);
}

export default MicrographPage;