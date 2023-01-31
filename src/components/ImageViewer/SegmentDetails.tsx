import { convertFileSrc, invoke } from '@tauri-apps/api/tauri'
import Image from 'next/image'
import useSWR from 'swr'

import Loading from '../layout/Loading'

import type { Segment } from '../../../src-tauri/bindings/Segment';
import { ArrowUpIcon } from '@heroicons/react/24/outline';

export type SegmentDetailsProps = {
    segment: Segment;
};

const SegmentDetails = ({ segment }: SegmentDetailsProps) => {
    const { data: image, error, isLoading } = useSWR(
        `/api/segments/${segment.uuid}/image`,
        () => fetcher(segment));


    if (error) return <div>Beim Laden ist ein Fehler aufgetreten: {error}</div>
    if (isLoading) return <Loading />

    return (
        <div className="grid grid-cols-2 gap-2">
            <div className="bg-black p-2 flex flex-col justify-center">
                {image && segment.width && segment.height && <Image src={image} width={segment.width} height={segment.height} alt="" />}
            </div>
            <div>
                {segment.status === 'new' ? <span>Segment wird analysiert</span> : (
                    <div className="text-sm py-4">
                        <p className="font-bold">Länge: </p><span>{segment.measured_length}</span>
                        <p className="font-bold">Breite: </p><span>{segment.measured_width}</span>
                        <p className="font-bold">Ausrichtung: </p><span>{segment.measured_angle}° </span>
                        <ArrowUpIcon className="w-10 m-auto" style={{ rotate: `${Math.floor(segment.measured_angle ?? 0) - 90}deg` }} />
                    </div>

                )}
            </div>
        </div>
    )
}

const fetcher = async (segment: Segment) => {
    const path = await import('@tauri-apps/api/path')
    const appDir = await path.appDataDir();

    const segmentPath = `${appDir}/micrographs/${segment.micrograph_id}/segments/${segment.filename}`;
    const image = await convertFileSrc(segmentPath);

    return image;
}

export default SegmentDetails