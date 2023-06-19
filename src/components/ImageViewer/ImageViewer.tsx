import { useEffect, useRef, useState } from "react";
import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";
import type { CachedMicrograph } from "../../../src-tauri/bindings/CachedMicrograph";
import type { CachedSegment } from "../../../src-tauri/bindings/CachedSegment";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import Image from "next/image";
import clsx from "clsx";
import { useRouter } from "next/router";

import SegmentDetails from "./SegmentDetails";

import useMouse from '@react-hook/mouse-position'
import { Button } from "../ui/button";

type ImageViewerProps = {
    micrograph: CachedMicrograph
    segments?: CachedSegment[]
}

const ImageViewer = ({ micrograph, segments }: ImageViewerProps) => {
    const viewerRef = useRef<HTMLImageElement>(null);
    const ref = useRef<HTMLDivElement>(null);

    const [viewerWidth, setViewerWidth] = useState<number>(0);
    const [viewerHeight, setViewerHeight] = useState<number>(0);
    const [showSegments, setShowSegments] = useState<boolean>(true);
    const [hoveredSegment, setHoveredSegment] = useState<CachedSegment | undefined>(undefined);


    const { x, y } = useMouse(ref, { enterDelay: 0, leaveDelay: 0 })
    const router = useRouter();

    useEffect(() => {
        // create event listener on windows resize
        window.addEventListener("resize", handleResize);

        // call handleResize to set initial state
        handleResize();

        // remove event listener on cleanup
        return () => window.removeEventListener("resize", handleResize);
    }, [segments, setViewerHeight, setViewerWidth])



    const handleResize = () => {
        if (viewerRef.current) {
            setViewerWidth(viewerRef.current.width);
            setViewerHeight(viewerRef.current.height);
        }
    }

    const overlayPosition = {
        top: (y && y <= (viewerHeight / 2)) ? y : undefined,
        bottom: (y && y > (viewerHeight / 2)) ? viewerHeight - y : undefined,
        left: (x && x <= (viewerWidth / 2)) ? x : undefined,
        right: (x && x > (viewerWidth / 2)) ? viewerWidth - x : undefined
    }

    return (
        <div className="space-y-4 absolute inset-0" ref={ref}>
            {micrograph && micrograph.display_img && micrograph.width && micrograph.height && (
                <div className="bg-gray-900 relative h-full max-h-screen">
                    {hoveredSegment && (
                        <div className="absolute rounded-md m-4 z-10 w-52 text-black  bg-neutral-300 bg-opacity-80 backdrop-blur-sm overflow-hidden shadow-xl" style={{ left: overlayPosition.left, right: overlayPosition.right, top: overlayPosition.top, bottom: overlayPosition.bottom }}>
                            <SegmentDetails segment={hoveredSegment} />
                        </div>
                    )}

                    <TransformWrapper>
                        <TransformComponent>
                            <Image src={convertFileSrc(micrograph.display_img)} alt={micrograph.name} width={micrograph.width} height={micrograph.height} ref={viewerRef} />
                            {segments && showSegments && segments.map(segment => {
                                if (segment.location_x === null || segment.location_y === null || segment.width === null || segment.height === null || micrograph.width === null || micrograph.height === null) return (<></>);
                                const { location_x: x, location_y: y, width, height } = segment;

                                const positionX = Math.floor((x / micrograph.width) * viewerWidth);
                                const positionY = Math.floor((y / micrograph.height) * viewerHeight);

                                const widthX = Math.floor((width / micrograph.width) * viewerWidth);
                                const heightY = Math.floor((height / micrograph.height) * viewerHeight);

                                return <div key={segment.uuid} onMouseOver={() => setHoveredSegment(segment)} onMouseOut={() => setHoveredSegment(undefined)} style={{ left: positionX, top: positionY, width: widthX, height: heightY }} className={clsx("absolute border rounded-sm transition-transform", segment.status == "Ok" ? "border-green-600" : "border-red-600", "hover:border-blue-600 hover:scale-110 hover:backdrop-brightness-200 cursor-pointer")}></div>
                            })
                            }
                        </TransformComponent>
                    </TransformWrapper>
                </div>
            )}

            {segments && segments.length > 0 && (
                <Button onClick={() => setShowSegments(!showSegments)} className="fixed top-0 left-0 m-8">Toggle Segments</Button>
            )
            }

        </div>
    );
};

export default ImageViewer;