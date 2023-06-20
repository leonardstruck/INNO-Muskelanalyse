import { useEffect, useRef, useState } from "react";
import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";
import type { CachedMicrograph } from "../../../src-tauri/bindings/CachedMicrograph";
import type { CachedSegment } from "../../../src-tauri/bindings/CachedSegment";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import Image from "next/image";
import clsx from "clsx";

import SegmentDetails from "./SegmentDetails";

import useMouse from '@react-hook/mouse-position'
import { Button } from "../ui/button";
import * as Popover from "@radix-ui/react-popover";

type ImageViewerProps = {
    micrograph: CachedMicrograph
    segments?: CachedSegment[]
}

const ImageViewer = ({ micrograph, segments }: ImageViewerProps) => {
    const viewerRef = useRef<HTMLImageElement>(null);

    const [viewerWidth, setViewerWidth] = useState<number>(0);
    const [viewerHeight, setViewerHeight] = useState<number>(0);
    const [showSegments, setShowSegments] = useState<boolean>(true);



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

    return (
        <>
            {micrograph && micrograph.display_img && micrograph.width && micrograph.height && (
                <>
                    <TransformWrapper>
                        <TransformComponent wrapperStyle={{ width: "100%", height: "100%", margin: 0, padding: 0 }}>
                            <Image src={convertFileSrc(micrograph.display_img)} alt={micrograph.name} width={micrograph.width} height={micrograph.height} ref={viewerRef} />
                            {segments && showSegments && segments.map(segment => {
                                if (segment.location_x === null || segment.location_y === null || segment.width === null || segment.height === null || micrograph.width === null || micrograph.height === null) return (<></>);
                                const { location_x: x, location_y: y, width, height } = segment;

                                const positionX = Math.floor((x / micrograph.width) * viewerWidth);
                                const positionY = Math.floor((y / micrograph.height) * viewerHeight);

                                const widthX = Math.floor((width / micrograph.width) * viewerWidth);
                                const heightY = Math.floor((height / micrograph.height) * viewerHeight);

                                return (
                                    <Popover.Root key={segment.uuid}>
                                        <Popover.Portal>
                                            <Popover.Content className="rounded-md m-4 z-10 w-52 text-black  bg-neutral-300 bg-opacity-80 backdrop-blur-sm overflow-hidden shadow-xl">
                                                <SegmentDetails segment={segment} />
                                            </Popover.Content>
                                        </Popover.Portal>
                                        <Popover.Trigger asChild>
                                            <div
                                                key={segment.uuid}
                                                style={{ left: positionX, top: positionY, width: widthX, height: heightY }}
                                                className={clsx(
                                                    "absolute border rounded-sm transition-transform",
                                                    segment.status == "Ok" ? "border-green-600" : "border-red-600", "hover:border-blue-600 hover:scale-110 hover:backdrop-brightness-200 cursor-pointer"
                                                )}
                                            />
                                        </Popover.Trigger>
                                    </Popover.Root>
                                )
                            })
                            }
                        </TransformComponent>
                    </TransformWrapper>
                </>
            )}

            {segments && segments.length > 0 && (
                <Button onClick={() => setShowSegments(!showSegments)} className="fixed top-0 left-0 m-8">Toggle Segments</Button>
            )
            }
        </>
    );
};

export default ImageViewer;